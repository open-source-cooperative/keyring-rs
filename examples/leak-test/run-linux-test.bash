#!/usr/bin/env bash
# The first argument to this script is the name of the credential store to test.
#
# To test all the Linux credential stores, run this three times:
#    bash run-linux-test.bash keyutils
#    bash run-linux-test.bash secret-service-sync
#    bash run-linux-test.bash secret-service-async
#
# By default, this "shows the sausage being made" as the test runs.
# If you don't want to see that, pipe the output of this script into
# a grep for the word `TEST`, as in:
#     bash run-linux-test.bash keyutils | grep TEST
# Then you will just see the test program output (on stderr)
# together with the results of the testing.
#
# You can also just ignore the output completely and look at the
# exit code: if it's 0, the test passed, if it's 1 it didn't.
#
# You must have installed `procdump` from the Microsoft SysInternals suite
# before you run this script. And make sure your Yama ptrace setting allows
# users to dump processes. (See https://stackoverflow.com/a/10163848/558006
# and let your search engine AI look for `how set value of yama ptrace`)
echo Building...
cargo build --example leak-test
echo Running...
# shellcheck disable=SC2086
cargo run --example leak-test -- $1 &
sleep 2
rm -f /tmp/leak-test.dmp.*
echo Dumping...
procdump -n 1 -s 0 -o leak-test /tmp/leak-test.dmp
# shellcheck disable=SC2181
if [ $? != 0 ]; then
  echo TEST ABORT - procdump failure
  echo Waiting for leak test to clean up...
  wait %1
  exit 1
fi
echo Grepping...
strings /tmp/leak-test.dmp.* | grep -H -n super-duper-password
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
