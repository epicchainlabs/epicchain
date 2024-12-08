#!/bin/bash

# Define start and end dates
START_DATE="2024-12-05"
END_DATE="2024-12-08"

# Convert dates to seconds since epoch
start=$(date -d $START_DATE +%s)
end=$(date -d $END_DATE +%s)

# Create 100 fake commits
for i in $(seq 1 150)
do
  # Random timestamp within the range
  random_timestamp=$(($start + $RANDOM % ($end - $start)))

  # Set the commit date
  export GIT_COMMITTER_DATE=$(date -d @$random_timestamp +"%Y-%m-%dT%H:%M:%S")
  export GIT_AUTHOR_DATE=$GIT_COMMITTER_DATE

  # Make a commit
  echo "EpicChain Lab's #$i" >> epicchain.txt
  git add epicchain.txt
  git commit -m "EpicChain Core - EpicChain Lab's"
done
