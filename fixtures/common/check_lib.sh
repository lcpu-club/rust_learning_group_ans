check_source() {
  # Check if the correct number of arguments are passed
  if [ "$#" -ne 2 ]; then
    echo "Usage: check_source <template_file> <code_file>"
    return 1
  fi

  local template_file="$1"
  local code_file="$2"
  local template_line code_line trimmed_code_line key value
  local -A kv_pairs
  local line_number=0

  # Open file descriptors for both files
  exec 3<"$template_file" || { echo "Error opening $template_file"; return 1; }
  exec 4<"$code_file" || { echo "Error opening $code_file"; exec 3<&-; return 1; }

  while true; do
    # Read lines from both files, removing carriage returns
    if ! IFS= read -r template_line <&3; then break; fi
    template_line=${template_line%$'\r'}

    if ! IFS= read -r code_line <&4; then
      if [[ -n $template_line && ! $template_line =~ ^[[:space:]]*$ ]]; then
        echo "Error: The code file is shorter than the template file."
        exec 3<&- 4<&-
        return 1
      fi
      # If the code file ends, check if the remaining template lines are all blank
      while IFS= read -r template_line <&3; do
        template_line=${template_line%$'\r'}
        if [[ -n $template_line && ! $template_line =~ ^[[:space:]]*$ ]]; then
          echo "Error: The code file is shorter than the template file."
          exec 3<&- 4<&-
          return 1
        fi
      done
      break
    fi
    code_line=${code_line%$'\r'}

    ((line_number++))

    if [[ $template_line =~ \ \//\ FIX\ ME(.*) ]]; then
      local suffix="${BASH_REMATCH[1]}"
      kv_pairs=()

      if [ -n "$suffix" ]; then
        for pair in $suffix; do
          IFS='=' read -r key value <<< "$pair"
          kv_pairs[$key]="$value"
        done
      fi

      trimmed_code_line=${code_line}
      trimmed_code_line=${trimmed_code_line##+([[:space:]])}
      trimmed_code_line=${trimmed_code_line%%+([[:space:]])}

      if [[ -n ${kv_pairs[limit]} && ${#trimmed_code_line} -gt ${kv_pairs[limit]} ]]; then
        echo "Error: Line $line_number in the code exceeds the character limit of ${kv_pairs[limit]}."
        exec 3<&- 4<&-
        return 1
      fi
    else
      if [[ "$template_line" != "$code_line" ]]; then
        echo "Error: Line $line_number does not match:"
        echo "Template: $template_line"
        echo "Code:     $code_line"
        exec 3<&- 4<&-
        return 1
      fi
    fi
  done

  # After the main comparison loop, check for trailing non-blank lines in the code file
  while IFS= read -r code_line <&4 || [[ -n $code_line ]]; do
    code_line=${code_line%$'\r'}
    if [[ -n $code_line && ! $code_line =~ ^[[:space:]]*$ ]]; then
      echo "Error: The code file has extra non-blank trailing line:"
      echo "Code: $code_line"
      exec 3<&- 4<&-
      return 1
    fi
  done

  # Close file descriptors
  exec 3<&- 4<&-

  return 0  # Success
}
