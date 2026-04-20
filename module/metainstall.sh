export KSU_HAS_METAMODULE="true"
export KSU_METAMODULE="magisk_mount"



handle_partition() {
	echo 0 > /dev/null ; true
}

mark_replace() {
	replace_target="$1"
	mkdir -p "$replace_target"
	setfattr -n trusted.overlay.opaque -v y "$replace_target"
}

install_module

handle_partitions() {
	partition="$1"
	
	if [ ! -d "$MODPATH/system/$partition" ]; then
		return
	fi
	
	if [ -L "/system/$partition" ] && [ -d "/$partition" ]; then
		ui_print "- Handle partition /$partition"
		ln -sf "./system/$partition" "$MODPATH/$partition"
	fi
}

handle_partitions vendor
handle_partitions system_ext
handle_partitions product
handle_partitions odm

