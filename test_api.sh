#!/bin/bash

BASE_URL="http://localhost:3000"

echo "=========================================="
echo "TEST 1: Health check"
echo "=========================================="
curl -s "$BASE_URL/health"
echo -e "\n"

echo "=========================================="
echo "TEST 2: Create 3 jobs"
echo "=========================================="

JOB1=$(curl -s -X POST "$BASE_URL/jobs" \
  -H "Content-Type: application/json" \
  -d '{"task_type": "email", "payload": "send email"}')
echo "Job 1: $JOB1"
ID1=$(echo $JOB1 | grep -o '"id":"[^"]*"' | cut -d'"' -f4)

JOB2=$(curl -s -X POST "$BASE_URL/jobs" \
  -H "Content-Type: application/json" \
  -d '{"task_type": "report", "payload": "generate report"}')
echo "Job 2: $JOB2"
ID2=$(echo $JOB2 | grep -o '"id":"[^"]*"' | cut -d'"' -f4)

JOB3=$(curl -s -X POST "$BASE_URL/jobs" \
  -H "Content-Type: application/json" \
  -d '{"task_type": "backup", "payload": "run backup"}')
echo "Job 3: $JOB3"
ID3=$(echo $JOB3 | grep -o '"id":"[^"]*"' | cut -d'"' -f4)

echo ""
echo "=========================================="
echo "TEST 3: Check immediate status (should be Running or Pending)"
echo "=========================================="
echo "Job 1:"
curl -s "$BASE_URL/jobs/$ID1"
echo -e "\n"

echo "=========================================="
echo "TEST 4: List all jobs"
echo "=========================================="
curl -s "$BASE_URL/jobs" | python3 -m json.tool 2>/dev/null || curl -s "$BASE_URL/jobs"
echo -e "\n"

echo "=========================================="
echo "TEST 5: Wait 3 seconds and check again"
echo "=========================================="
echo "Waiting 3 seconds..."
sleep 3

echo "Job 1 (should be Completed):"
curl -s "$BASE_URL/jobs/$ID1"
echo -e "\n"

echo "Job 2:"
curl -s "$BASE_URL/jobs/$ID2"
echo -e "\n"

echo "Job 3:"
curl -s "$BASE_URL/jobs/$ID3"
echo -e "\n"

echo "=========================================="
echo "TEST 6: Query non-existent job"
echo "=========================================="
curl -s "$BASE_URL/jobs/00000000-0000-0000-0000-000000000000"
echo -e "\n"

echo "=========================================="
echo "TEST 7: List all final jobs"
echo "=========================================="
curl -s "$BASE_URL/jobs" | python3 -m json.tool 2>/dev/null || curl -s "$BASE_URL/jobs"
echo -e "\n"

echo "=========================================="
echo "TESTS COMPLETE"
echo "=========================================="
