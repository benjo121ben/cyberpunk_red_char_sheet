#!/usr/bin/env sh
cargo leptos build --release &&
t_dir=cyberpunk_app &&
rm -r $t_dir
mkdir $t_dir &&
mkdir $t_dir/dev_saves &&
mkdir $t_dir/gear &&
cp -r target/site/. $t_dir/site/ &&
# this was at some point necessary, might become relevant again who knows
# mv $t_dir/site/pkg/cp_red_char_sheet.wasm $t_dir/site/pkg/cp_red_char_sheet_bg.wasm &&
cp -r gear/. $t_dir/gear/ &&
cp character.json $t_dir/dev_saves/character.json &&
cp target/release/cp_red_char_sheet $t_dir/cp_red_char_sheet &&
cp Cargo_deploy.toml $t_dir/Cargo.toml && # cargo deploy sets site different
tar czf cyberpunk-app.tar.gz -C $t_dir . &&
echo project built and zipped into tar file