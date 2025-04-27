#!/usr/bin/env sh
cargo leptos build --release &&
t_dir=cyberpunk_app &&
rm -r $t_dir
mkdir $t_dir &&
mkdir $t_dir/dev_saves &&
mkdir $t_dir/gear &&
cp -r target/site/. $t_dir/site/ &&
mv $t_dir/site/pkg/cyberpunk_app.wasm $t_dir/site/pkg/cyberpunk_app_bg.wasm &&
cp -r gear/. $t_dir/gear/ &&
cp character.json $t_dir/character.json &&
cp target/release/cp_red_char_sheet $t_dir/cp_red_char_sheet &&
cp Cargo.toml $t_dir/Cargo.toml &&
tar czf cyberpunk-app.tar.gz -C $t_dir . &&
echo project built and zipped into tar file