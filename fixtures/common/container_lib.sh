#!/bin/bash

BUSCTL="busctl --user"

create_cgroup_systemd() {
  local slice_name=$1

  if [ -z "$slice_name" ]; then
    echo "Usage: create_cgroup <slice_name>" >&2
    return 1
  fi

  local job
  job=$($BUSCTL call org.freedesktop.systemd1 \
    /org/freedesktop/systemd1 \
    org.freedesktop.systemd1.Manager \
    StartTransientUnit 'ssa(sv)a(sa(sv))' \
    "${slice_name}" \
    replace \
    2 \
      CPUAccounting b 1 \
      MemoryAccounting b 1 \
    0)
  echo "Create cgroup job: $job" >&2
}

delete_cgroup_systemd() {
  local slice_name=$1

  if [ -z "$slice_name" ]; then
    echo "Usage: delete_cgroup <slice_name>" >&2
    return 1
  fi

  local job
  job=$($BUSCTL call org.freedesktop.systemd1 \
    /org/freedesktop/systemd1 \
    org.freedesktop.systemd1.Manager \
    StopUnit 'ss' \
    "${slice_name}" \
    replace)
  echo "Delete cgroup job: $job" >&2
}

get_cgroup_path_systemd() {
  slice_name=$1

  local dbus_path
  dbus_path=$($BUSCTL call org.freedesktop.systemd1 \
    /org/freedesktop/systemd1 \
    org.freedesktop.systemd1.Manager \
    LoadUnit s \
    "${slice_name}")
  dbus_path=$(echo "$dbus_path" | awk '{print $2}')
  dbus_path=${dbus_path:1:-1}

  local cgroup_path
  cgroup_path=$($BUSCTL get-property org.freedesktop.systemd1 \
    "$dbus_path" \
    org.freedesktop.systemd1.Slice \
    ControlGroup)
  cgroup_path=$(echo "$cgroup_path" | awk '{print $2}')
  cgroup_path=${cgroup_path:1:-1}

  echo "/sys/fs/cgroup$cgroup_path"
}

read_cgroup_metrics() {
  local cgroup_path=$1

  if [ -z "$cgroup_path" ]; then
    echo "Usage: read_cgroup_v2_data <cgroup_path>"
    return 1
  fi

  if [ ! -d "$cgroup_path" ]; then
    echo "Error: Provided path is not a directory"
    return 1
  fi

  local cpu_usage_ms
  local memory_peak_kb

  if [ -f "$cgroup_path/cpu.stat" ]; then
    cpu_usage_ms=$(awk '/usage_usec/ {print $2/1000}' "$cgroup_path/cpu.stat")
  else
    echo "cpu.stat file does not exist in the provided cgroup path." >&2
    return 1
  fi

  if [ -f "$cgroup_path/memory.peak" ]; then
    memory_peak_kb=$(awk '{print $1/1024}' "$cgroup_path/memory.peak")
  else
    echo "memory.peak file does not exist in the provided cgroup path." >&2
    memory_peak_kb=0
  fi

  echo "{ \"cpu\": $cpu_usage_ms, \"mem\": $memory_peak_kb }"
}

run_container() {
  local timeout
  local log_stdout
  local log_stderr
  local log_limit
  while [[ $# -gt 0 ]]; do
    case $1 in
      --)
        shift
        break
        ;;
      -t|--timeout)
        timeout="$2"
        shift
        shift
        ;;
      -o|--stdout)
        log_stdout="$2"
        shift
        shift
        ;;
      -e|--stderr)
        log_stderr="$2"
        shift
        shift
        ;;
      -n|--tail)
        log_limit="$2"
        shift
        shift
        ;;
      *)
        echo "Unknown option $1" >&2
        return 1
        ;;
    esac
  done
  if [ -z "$timeout" ]; then
    echo "Error: --timeout is required"  >&2
    return 1
  fi

  local random_id
  local cgroup_name
  random_id=$(head /dev/urandom | tr -dc A-Za-z0-9 | head -c 16 ; echo '')
  cgroup_name="sandbox-$random_id.slice"
  create_cgroup_systemd "$cgroup_name"
  if [ $? -ne 0 ]; then
    echo "Error: Failed to create cgroup" >&2
    return 1
  fi

  local podman_args=("--cgroup-parent=$cgroup_name" "$@")
  echo "Running podman with args: ${podman_args[*]}" >&2

  local container_id
  container_id=$(podman run -d "${podman_args[@]}")
  if [ -z "$container_id" ]; then
    echo "Error: Failed to start podman container" >&2
    delete_cgroup_systemd "$cgroup_name"
    return 1
  fi
  echo "Container ID: $container_id" >&2

  local exit_code
  exit_code=$(timeout "$timeout" podman wait "$container_id")
  if [ $? -ne 0 ]; then
    echo "Timeout reached. Killing container" >&2
    podman kill "$container_id" > /dev/null || true
    exit_code=124
  else
    echo "Container exited with code: $exit_code" >&2
  fi

  if [ -n "$log_stdout" ] || [ -n "$log_stderr" ]; then
    log_stdout=${log_stdout:-/dev/null}
    log_stderr=${log_stderr:-/dev/null}
    if [ -n "$log_limit" ]; then
      podman logs --tail "$log_limit" "$container_id" > "$log_stdout" 2> "$log_stderr"
    else
      podman logs "$container_id" > "$log_stdout" 2> "$log_stderr"
    fi
  fi

  podman rm "$container_id" > /dev/null

  local cgroup_path
  local metrics
  cgroup_path=$(get_cgroup_path_systemd "$cgroup_name")
  metrics=$(read_cgroup_metrics "$cgroup_path")
  delete_cgroup_systemd "$cgroup_name"
  echo "$metrics"
  return "$exit_code"
}
