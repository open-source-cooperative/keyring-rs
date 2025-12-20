#!/usr/bin/env bash
# When run with no arguments, this will test the `keychain` credential store.
#
# You can test another credential store by giving its name as the first
# argument to this script. For example, you might want to specify the
# sample store (which does leak) to make sure this test is working.
#
# By default, this "shows the sausage being made" as the test runs.
# If you don't want to see that, pipe the out of this script into
# a grep for the word `TEST`, as in:
#     bash run-macos-test.bash | grep TEST
# Then you will just see the test program output (on stderr)
# together with the results of the testing.
echo Building...
cargo build --example leak-test
echo Running...
# shellcheck disable=SC2086
DELAY_SECS=5 cargo run --example leak-test -- ${1:keychain} &
sleep 2
echo Dumping...
rm -f /tmp/leak-test.modified.dmp
command="process save-core -s modified-memory /tmp/leak-test.modified.dmp"
lldb --attach-name leak-test --batch --one-line "$command" | grep -v "^Saving"
echo Grepping...
strings - /tmp/leak-test.modified.dmp | grep "super-duper-password"
# shellcheck disable=SC2181
if [ $? == 0 ]; then
  echo TEST FAILED
else
  echo TEST SUCEEDED.
fi
