<?php <<__EntryPoint>> function main() {
var_dump(bind_textdomain_codeset('',''));
var_dump(bind_textdomain_codeset('messages', "UTF-8"));

echo "Done\n";
}
