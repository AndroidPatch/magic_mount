export KSU_HAS_METAMODULE="true"
export KSU_METAMODULE="magisk_mount"

install_module

handle_partition() {
	partition="$1"
	
	if [ ! -d "$MODPATH/system/$partition" ]; then
		return
	fi
	
	if [ -L "/system/$partition" ] && [ -d "/$partition" ]; then
		ui_print "- Handle partition /$partition"
		ln -sf "./system/$partition" "$MODPATH/$partition"
	fi
}

handle_partition system_ext
handle_partition vendor
handle_partition product

