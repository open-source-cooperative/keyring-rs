# run this in a git bash shell
echo Building...
cargo build --example leak-test
echo Running...
cargo run --example leak-test &
sleep 2
echo Dumping...
/c/Windows/procdump64a.exe -ma -o keyring-test /tmp/keyring-test.dmp
echo Grepping...
/c/Windows/strings64a.exe /tmp/keyring-test.dmp | grep -H -n super-duper-password
# shellcheck disable=SC2181
if [ $? == 0 ]; then
  echo TEST FAILED
else
  echo TEST SUCEEDED.
fi
