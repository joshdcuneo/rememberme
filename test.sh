if [ "$1" = "list" ]; then
  echo "Listing entries"
  curl localhost:8080/entries
elif [ "$1" = "show" ]; then
  echo "Showing entry"
  curl localhost:8080/entries/test
elif [ "$1" = "create" ]; then
  echo "Creating entry"
  curl localhost:8080/entries \
    -X POST \
    -H "Content-Type: application/json" \
    -d '{"name":"test", "description":"test entry", "slug": "test"}'
elif [ "$1" = "update" ]; then
  echo "Updating entry"
  curl localhost:8080/entries/test \
    -X PUT \
    -H "Content-Type: application/json" \
    -d '{"name":"test", "description":"test entry", "slug": "test"}'
elif [ "$1" = "delete" ]; then
  echo "Deleting entry"
  curl localhost:8080/entries/test \
    -X DELETE
else
  echo "Usage: $0 [list|show|create|update|delete]"
fi