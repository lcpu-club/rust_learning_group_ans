#!/bin/bash

generate_summary() {
  local metrics=$1
  local exit_code=$2
  local stdout=$3
  local stderr=$4
  local stdout_content
  local stderr_content
  # load first 50 lines of stdout file and stderr file
  if [ -f "$stdout" ]; then
    stdout_content=$(head -n 50 "$stdout")
  else
    stdout_content="stdout not found"
  fi
  if [ -f "$stderr" ]; then
    stderr_content=$(head -n 50 "$stderr")
  else
    stderr_content="stderr not found"
  fi
  echo '## Command Metrics'
  echo '- **Exit Code**: `'"$exit_code"'`'
  echo '- **CPU Usage**: `'"$(echo "$metrics" | jq -r '.cpu')"' ms`'
  echo '- **Memory Usage**: `'"$(echo "$metrics" | jq -r '.mem')"'` KB'
  echo ''
  echo '## Command output'
  echo '### stdout'
  echo ''
  echo '```'
  echo "$stdout_content"
  echo '```'
  echo ''
  echo '### stderr'
  echo ''
  echo '```'
  echo "$stderr_content"
  echo '```'
}

generate_test() {
  local name=$1
  local score=$2
  local score_scale=$3
  local status=$4
  local summary=$5
  jq -n --arg name "$name" --argjson score "$score" --argjson score_scale "$score_scale" --arg status "$status" --arg summary "$summary" \
    '{name: $name, score: $score, scoreScale: $score_scale, status: $status, summary: $summary}'
}

generate_job() {
  local name=$1
  local score=$2
  local score_scale=$3
  local status=$4
  local summary=$5
  jq -n --arg name "$name" --argjson score "$score" --argjson score_scale "$score_scale" --arg status "$status" --arg summary "$summary" \
    '{name: $name, score: $score, scoreScale: $score_scale, status: $status, summary: $summary, tests: []}'  
}

append_test_to_job() {
  local job=$1
  local test=$2
  jq --argjson test "$test" '.tests += [$test]' <<<"$job"
}

generate_details() {
  local summary=$1
  jq -n --arg summary "$summary" '{jobs: [], summary: $summary}'
}

append_job_to_details() {
  local details=$1
  local job=$2
  jq --argjson job "$job" '.jobs += [$job]' <<<"$details"
}
