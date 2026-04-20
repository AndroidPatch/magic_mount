export KSU_HAS_METAMODULE="true"
export KSU_METAMODULE="magisk_mount"

install_module

handle_partition() {
	echo 0 > /dev/null ; true
}

mark_replace() {
	replace_target="$1"
	mkdir -p "$replace_target"
	setfattr -n trusted.overlay.opaque -v y "$replace_target"
}

handle_partitions() {
    PARTITION="$1"
    REQUIRE_SYMLINK="$2"
    if [ ! -e "$MODPATH/system/$PARTITION" ]; then
        # no partition found
        return;
    fi

    if [ "$REQUIRE_SYMLINK" = "false" ] || [ -L "/system/$PARTITION" ] && [ "$(readlink -f "/system/$PARTITION")" = "/$PARTITION" ]; then
        ui_print "- Handle partition /$PARTITION"
        ln -sf "./system/$PARTITION" "$MODPATH/$PARTITION"
    fi
}

handle_partitions vendor true
handle_partitions system_ext true
handle_partitions product true
handle_partitions odm false

