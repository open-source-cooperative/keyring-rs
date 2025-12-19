# run this in a bash shell
echo Building...
cargo build --example leak-test
echo Running...
DELAY_SECS=5 cargo run --example leak-test &
sleep 2
echo Dumping...
command="process save-core -s modified-memory /tmp/keyring-test.modified.dmp"
lldb --attach-name leak-test --batch --one-line "$command" | grep -v "^Saving"
echo Grepping...
strings - /tmp/keyring-test.modified.dmp | grep "super-duper-password"
# shellcheck disable=SC2181
if [ $? == 0 ]; then
  echo TEST FAILED
else
  echo TEST SUCEEDED.
fi
