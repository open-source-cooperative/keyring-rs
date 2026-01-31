#!/usr/bin/env bash
# Run this in a git bash shell.
#
# When run with no arguments, this will test the `windows` credential store.
#
# You can test another credential store by giving its name as the first
# argument to this script. For example, you might want to specify the
# sample store (which does leak) to make sure this test is working.
#
# By default, this "shows the sausage being made" as the test runs.
# If you don't want to see that, pipe the output of this script into
# a grep for the word `TEST`, as in:
#     bash run-windows-test.bash | grep TEST
# Then you will just see the test program output (on stderr)
# together with the results of the testing.
#
# You can also just ignore the output completely and look at the
# exit code: if it's 0, the test passed, if it's 1 it didn't.
#
# You must have installed procdump64a and strings64a from
# the Microsoft SysInternals suite before you run this.
echo Building...
cargo build --example leak-test
echo Running...
# shellcheck disable=SC2086
DELAY_SECS=5 cargo run --example leak-test -- ${1:-windows} &
sleep 2
rm -f /tmp/keyring-test.dmp
echo Dumping...
/c/Windows/procdump64a.exe -ma -o leak-test /tmp/leak-test.dmp
# shellcheck disable=SC2181
if [ $? != 1 ]; then
  echo TEST ABORT - procdump failure
  echo Waiting for leak test to clean up...
  wait %1
  exit 1
fi
echo Grepping...
/c/Windows/strings64a.exe /tmp/leak-test.dmp | grep -H -n super-duper-password
# shellcheck disable=SC2181
if [ $? == 0 ]; then
  exit_code=1
  echo TEST FAILED
else
  exit_code=0
  echo TEST SUCCEEDED.
fi
echo Waiting for leak test to clean up...
wait %1
exit $exit_code
