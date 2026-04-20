export KSU_HAS_METAMODULE="true"
export KSU_METAMODULE="magisk_mount"

install_module

handle_partition() {
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

handle_partition vendor true
handle_partition system_ext true
handle_partition product true
handle_partition odm false

