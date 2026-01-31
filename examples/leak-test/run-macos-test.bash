#!/usr/bin/env bash
# When run with no arguments, this will test the `keychain` credential store.
#
# You can test another credential store by giving its name as the first
# argument to this script. For example, you might want to specify the
# sample store (which does leak) to make sure this test is working.
#
# By default, this "shows the sausage being made" as the test runs.
# If you don't want to see that, pipe the output of this script into
# a grep for the word `TEST`, as in:
#     bash run-macos-test.bash | grep TEST
# Then you will just see the test program output (on stderr)
# together with the results of the testing.
#
# You can also just ignore the output completely and look at the
# exit code: if it's 0, the test passed, if it's 1 it didn't.
echo Building...
cargo build --example leak-test
echo Running...
# shellcheck disable=SC2086
cargo run --example leak-test -- ${1:-keychain} &
sleep 2
rm -f /tmp/leak-test.modified.dmp /tmp/lldb-output.log
echo Dumping...
command="process save-core -s modified-memory /tmp/leak-test.modified.dmp"
lldb --attach-name leak-test --batch --one-line "$command" > /tmp/lldb-output.log
# shellcheck disable=SC2181
if [ $? != 0 ]; then
  echo TEST ABORT - lldb failure
  echo Waiting for leak test to clean up...
  wait %1
  exit 1
fi
echo Grepping...
strings - /tmp/leak-test.modified.dmp | grep "super-duper-password"
# shellcheck disable=SC2181
if [ $? == 0 ]; then
  exitcode=1
  echo TEST FAILED
else
  exitcode=0
  echo TEST SUCCEEDED.
fi
echo Waiting for leak test to clean up...
wait %1
exit $exitcode
