install-tools:
	cargo install slint-tr-extractor
	cargo install slint-viewer 

extract-string:
	@(find -name \*.slint | xargs slint-tr-extractor -o  singularity_strings.pot)

start-viewer:
	slint-viewer  --auto-reload  singularity_ui/ui/app-window.slint