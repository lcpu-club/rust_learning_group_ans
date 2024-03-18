#!/bin/bash

script_dir=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
source "$script_dir/container_lib.sh"
source "$script_dir/check_lib.sh"
source "$script_dir/judge_lib.sh"
start_time=$(date +%s)

details=$(cat "$GLUE_DETAILS")

if [ -f problem/template.rs ]; then
    score=100
    status="Success"
    summary=$(cat <<EOF
Source check passed
EOF
)
    msg=$(check_source ./problem/template.rs ./solution/main.rs)
    if [ $? -ne 0 ]; then
        score=0
        status="Wrong Answer"
        summary=$(cat <<EOF
Source check failed:
\`\`\`
$msg
\`\`\`
EOF
)
    fi
    compile_job=$(generate_job "Source Check" "$score" 0 "$status" "$summary")
    details=$(append_job_to_details "$details" "$compile_job")
    echo "$details" >"$GLUE_DETAILS"

    if [ $score -eq 0 ]; then
        echo score=0 >"$GLUE_REPORT"
        echo status="$status" >"$GLUE_REPORT"
        echo message="Source check failed" >"$GLUE_REPORT"
        echo commit=1 >"$GLUE_REPORT"
        exit 0
    fi
fi

echo score=0 >"$GLUE_REPORT"
echo status=Running >"$GLUE_REPORT"
echo "message=Compiling solution" >"$GLUE_REPORT"
echo commit=1 >"$GLUE_REPORT"

cp -r problem/template combined_solution

if [ -f problem/.oj-merge ] && [ -f problem/.source.rs ]; then
    echo "message=Merging source" >"$GLUE_REPORT"
    cat problem/.source.rs >>combined_solution/src/main.rs
    echo "// -- Source of the problem ends here --" >>combined_solution/src/main.rs
fi

cat solution/main.rs >>combined_solution/src/main.rs

cd combined_solution || exit 1
metrics=$(run_container -t 10 -o ../build.out -e ../build.err -- -v .:/solution -w /solution docker.io/rust:bookworm cargo build -r --features judge)
exit_code=$?
artifact=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[].targets[] | select( .kind | map(. == "bin") | any ) | .name')
binary=../combined_solution/target/release/$artifact
summary=$(generate_summary "$metrics" "$exit_code" ../build.out ../build.err)
score=100
status="Success"
if [ $exit_code -ne 0 ]; then
    score=0
    status="Compile Error"
fi
compile_job=$(generate_job "Compile" "$score" 0 "$status" "$summary")
details=$(append_job_to_details "$details" "$compile_job")
echo "$details" >"$GLUE_DETAILS"

if [ $exit_code -ne 0 ]; then
    echo score=0 >"$GLUE_REPORT"
    echo status="$status" >"$GLUE_REPORT"
    echo message="Failed to compile solution" >"$GLUE_REPORT"
    echo commit=1 >"$GLUE_REPORT"
    exit 0
fi

cd ../problem || exit 1
N=$(find . -maxdepth 1 -name 'test_*.in' | wc -l)
have_score_files=$(find . -maxdepth 1 -name 'test_*.score' | wc -l)
total_score=0
declare -A score_per_test
if [ "$have_score_files" -ne 0 ]; then
    for i in $(seq 1 "$N"); do
        score_file=test_$i.score
        score_per_test[$i]=$(cat "$score_file")
        if [ $? -ne 0 ]; then
            echo "Failed to read score file $score_file"
            exit 1
        fi
        total_score=$(($total_score + ${score_per_test[$i]}))
    done
    if [ $total_score -ne 100 ]; then
        echo "Total score is not 100"
        exit 1
    fi
else
    score=$((100 / $N))
    for i in $(seq 1 $(($N - 1))); do
        score_per_test[$i]=$score
        total_score=$(($total_score + $score))
    done
    score_per_test[$N]=$((100 - $total_score))
fi
main_job=$(generate_job "Main" 100 100 "" "")
job_start_time=$(date +%s)
total_score=0
final_status=Accepted
for i in $(seq 1 "$N"); do
    echo score=$total_score >"$GLUE_REPORT"
    echo status=Running >"$GLUE_REPORT"
    echo "message=Running on test $i" >"$GLUE_REPORT"
    # echo metrics='{"cpu":1,"mem":1024}' >$GLUE_REPORT
    echo commit=1 >"$GLUE_REPORT"
    IN=test_$i.in
    ANS=test_$i.ans
    OUT=test_$i.out
    ERR=test_$i.err
    touch "$OUT"
    touch "$ERR"
    # "./$binary" < "$IN" > "$OUT"
    case_metrics=$(run_container -t 10 -- \
      -v "./$IN":/input:ro \
      -v "./$OUT":/stdout \
      -v "./$ERR":/stderr \
      -v "$binary":/exe \
      docker.io/debian:bookworm sh -c "/exe </input >/stdout 2>/stderr")
    case_exit_code=$?
    case_total_score=${score_per_test[$i]}
    case_score=0
    case_status="Accepted"
    if [ $case_exit_code -ne 0 ]; then
        case_status="Runtime Error"
        final_status=$case_status
    else
        diff -q "$ANS" "$OUT" >/dev/null
        if [ $? -ne 0 ]; then
            case_status="Wrong Answer"
            final_status=$case_status
        else
            case_score=100
        fi
    fi
    total_score=$(($total_score + $case_score * $case_total_score / 100))
    case_summary=$(generate_summary "$case_metrics" "$case_exit_code" "$OUT" "$ERR")
    case_test=$(generate_test "Test $i" "$case_score" "$case_total_score" "$case_status" "$case_summary")
    main_job=$(append_test_to_job "$main_job" "$case_test")
done
job_end_time=$(date +%s)
job_duration=$(($job_end_time - $job_start_time))
job_summary="Job finished in $job_duration seconds"
main_job=$(jq --arg summary "$job_summary" '.summary=$summary' <<<"$main_job")
main_job=$(jq --argjson score "$total_score" '.score=$score' <<<"$main_job")
main_job=$(jq --arg status "$final_status" '.status=$status' <<<"$main_job")
details=$(append_job_to_details "$details" "$main_job")

end_time=$(date +%s)
duration=$(($end_time - $start_time))
summary="Judge finished in $duration seconds"
details=$(jq --arg summary "$summary" '.summary=$summary' <<<"$details")
echo "$details" >"$GLUE_DETAILS"

echo score=$total_score >"$GLUE_REPORT"
echo status="$final_status" >"$GLUE_REPORT"
echo message=OK >"$GLUE_REPORT"
# echo metrics='{"cpu":1,"mem":1024}' >$GLUE_REPORT
echo commit=1 >"$GLUE_REPORT"
