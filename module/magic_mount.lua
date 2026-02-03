
local M = {}



function M.version() --module version(recommended addition)
return 1
end

function M.post_fs_data(superkey)
    info("post_fs_data called")

    
end

function M.post_mount(superkey)
    info("post_mount called")
    if superkey=="kernelsu" then
        mount_source= "KSU"
    else
        mount_source= "APatch"
    end
    local libmagisk_mount = require("libmagic_mount")
    libmagisk_mount.magic_mount(mount_source)

end
function M.service(superkey)
    info("service called")
end

function M.action()
    info("action called")
    print("this is action function")
    os.execute("sleep 2")
end
return M