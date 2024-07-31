#!/usr/bin/env bash
# Written in [Amber](https://amber-lang.com/)
# version: 0.3.4-alpha
# date: 2024-07-31 10:04:32
function has_failed__25_v0 {
    local command=$1
    eval ${command} > /dev/null 2>&1;
    __AS=$?
    __AF_has_failed25_v0=$(echo $__AS '!=' 0 | bc -l | sed '/\./ s/\.\{0,1\}0\{1,\}$//');
    return 0
}
function exit__26_v0 {
    local code=$1
    exit "${code}";
    __AS=$?
}
function includes__27_v0 {
    local arr=("${!1}")
    local value=$2
    for v in "${arr[@]}"
do
        if [ $([ "_${v}" != "_${value}" ]; echo $?) != 0 ]; then
            __AF_includes27_v0=1;
            return 0
fi
done
    __AF_includes27_v0=0;
    return 0
}
function get_os__46_v0 {
    __AMBER_VAL_0=$(uname -s);
    __AS=$?;
if [ $__AS != 0 ]; then
        echo "Failed to determine OS type (using \`uname\` command)."
        echo "Please try again or make sure you have it installed."
        exit__26_v0 1;
        __AF_exit26_v0__8_9=$__AF_exit26_v0;
        echo $__AF_exit26_v0__8_9 > /dev/null 2>&1
fi;
    local os_type="${__AMBER_VAL_0}"
    if [ $([ "_${os_type}" != "_Darwin" ]; echo $?) != 0 ]; then
        __AF_get_os46_v0="apple-darwin";
        return 0
fi
    if [ $([ "_${os_type}" == "_Linux" ]; echo $?) != 0 ]; then
        echo "Unsupported OS type: ${os_type}"
        echo "Please try again or use another download method."
        exit__26_v0 1;
        __AF_exit26_v0__16_9=$__AF_exit26_v0;
        echo $__AF_exit26_v0__16_9 > /dev/null 2>&1
fi
    has_failed__25_v0 "ls -l /lib | grep libc.musl";
    __AF_has_failed25_v0__19_12=$__AF_has_failed25_v0;
    if [ $(echo  '!' $__AF_has_failed25_v0__19_12 | bc -l | sed '/\./ s/\.\{0,1\}0\{1,\}$//') != 0 ]; then
        __AF_get_os46_v0="unknown-linux-musl";
        return 0
fi
    __AF_get_os46_v0="unknown-linux-gnu";
    return 0
}
function get_arch__47_v0 {
    __AMBER_VAL_1=$(uname -m);
    __AS=$?;
if [ $__AS != 0 ]; then
        echo "Failed to determine architecture."
        echo "Please try again or use another download method."
        exit__26_v0 1;
        __AF_exit26_v0__31_9=$__AF_exit26_v0;
        echo $__AF_exit26_v0__31_9 > /dev/null 2>&1
fi;
    local arch_type="${__AMBER_VAL_1}"
    __AMBER_ARRAY_0=("arm64" "aarch64");
    includes__27_v0 __AMBER_ARRAY_0[@] "${arch_type}";
    __AF_includes27_v0__34_16=$__AF_includes27_v0;
    local arch=$(if [ $__AF_includes27_v0__34_16 != 0 ]; then echo "aarch64"; else echo "x86_64"; fi)
    __AF_get_arch47_v0="${arch}";
    return 0
}
function get_home__48_v0 {
    __AMBER_VAL_2=$(echo $HOME);
    __AS=$?;
if [ $__AS != 0 ]; then
        echo "User installation requested, but unable to retrieve home directory from $HOME environment."
        exit__26_v0 1;
        __AF_exit26_v0__44_9=$__AF_exit26_v0;
        echo $__AF_exit26_v0__44_9 > /dev/null 2>&1
fi;
    local home="${__AMBER_VAL_2}"
    if [ $([ "_${home}" != "_" ]; echo $?) != 0 ]; then
        echo "User installation requested, but unable to find home directory."
        exit__26_v0 1;
        __AF_exit26_v0__48_9=$__AF_exit26_v0;
        echo $__AF_exit26_v0__48_9 > /dev/null 2>&1
fi
    __AF_get_home48_v0="${home}";
    return 0
}
function get_bins_folder__49_v0 {
    local user_only=$1
    if [ ${user_only} != 0 ]; then
        get_home__48_v0 ;
        __AF_get_home48_v0__55_18="${__AF_get_home48_v0}";
        __AF_get_bins_folder49_v0="${__AF_get_home48_v0__55_18}/.local/bin";
        return 0
else
        local bins_folder="/usr/local/bin"
        test -d "${bins_folder}" > /dev/null 2>&1;
        __AS=$?;
if [ $__AS != 0 ]; then
            sudo mkdir -p "${bins_folder}" > /dev/null 2>&1;
            __AS=$?;
if [ $__AS != 0 ]; then
                echo "Failed to create ${bins_folder} directory."
                exit__26_v0 1;
                __AF_exit26_v0__62_17=$__AF_exit26_v0;
                echo $__AF_exit26_v0__62_17 > /dev/null 2>&1
fi
fi
        __AF_get_bins_folder49_v0="${bins_folder}";
        return 0
fi
}
function get_latest_release_tag__50_v0 {
    local tag_url="https://api.github.com/repos/towry/npmpink/releases/latest"
    __AMBER_VAL_3=$(curl -sL "${tag_url}");
    __AS=$?;
if [ $__AS != 0 ]; then
__AF_get_latest_release_tag50_v0=''
return $__AS
fi;
    local tag_json="${__AMBER_VAL_3}"
    __AMBER_VAL_4=$(echo "$tag_json"         | grep -Eo "tag_name\"[^\"]*\"([^\"]+)\""         | grep -Eo "\"[^\"]+\"$"         | grep -Eo "[^\"\s]+");
    __AS=$?;
if [ $__AS != 0 ]; then
__AF_get_latest_release_tag50_v0=''
return $__AS
fi;
    local tag="${__AMBER_VAL_4}"
    __AF_get_latest_release_tag50_v0="${tag}";
    return 0
}
__0_archive="npk.tar.gz"
args=("$@")
    get_os__46_v0 ;
    __AF_get_os46_v0__83_12="${__AF_get_os46_v0}";
    os="${__AF_get_os46_v0__83_12}"
    get_arch__47_v0 ;
    __AF_get_arch47_v0__84_14="${__AF_get_arch47_v0}";
    arch="${__AF_get_arch47_v0__84_14}"
    includes__27_v0 args[@] "--user";
    __AF_includes27_v0__86_27=$__AF_includes27_v0;
    user_only_install=$__AF_includes27_v0__86_27
    get_bins_folder__49_v0 ${user_only_install};
    __AF_get_bins_folder49_v0__87_21="${__AF_get_bins_folder49_v0}";
    bins_folder="${__AF_get_bins_folder49_v0__87_21}"
    has_failed__25_v0 "curl -V";
    __AF_has_failed25_v0__90_6=$__AF_has_failed25_v0;
    if [ $__AF_has_failed25_v0__90_6 != 0 ]; then
        echo "Curl is not installed on your system."
        echo "Please install \`curl\` and try again."
        exit__26_v0 1;
        __AF_exit26_v0__93_7=$__AF_exit26_v0;
        echo $__AF_exit26_v0__93_7 > /dev/null 2>&1
fi
    echo "Installing npmpink..."
    if [ ${user_only_install} != 0 ]; then
        mkdir -p "${bins_folder}" > /dev/null 2>&1;
        __AS=$?;
if [ $__AS != 0 ]; then
            echo "Failed to create directory bin at ${bins_folder}."
            exit__26_v0 1;
            __AF_exit26_v0__100_11=$__AF_exit26_v0;
            echo $__AF_exit26_v0__100_11 > /dev/null 2>&1
fi
fi
    get_latest_release_tag__50_v0 ;
    __AS=$?;
if [ $__AS != 0 ]; then
        echo "Failed to get the latest release tag."
        echo "Please try again or use another download method."
        exit__26_v0 1;
        __AF_exit26_v0__107_7=$__AF_exit26_v0;
        echo $__AF_exit26_v0__107_7 > /dev/null 2>&1
fi;
    __AF_get_latest_release_tag50_v0__104_12="${__AF_get_latest_release_tag50_v0}";
    tag="${__AF_get_latest_release_tag50_v0__104_12}"
    url="https://github.com/towry/npmpink/releases/download/${tag}/npk-${tag}-${arch}-${os}.tar.gz"
    cd /tmp;
    __AS=$?;
if [ $__AS != 0 ]; then

exit $__AS
fi
    curl --styled-output -# -L -o "${__0_archive}" "${url}" > /dev/null 2>&1;
    __AS=$?;
if [ $__AS != 0 ]; then
        echo "Curl failed to download npmpink."
        echo "Something went wrong. Please try again later."
        exit__26_v0 1;
        __AF_exit26_v0__119_7=$__AF_exit26_v0;
        echo $__AF_exit26_v0__119_7 > /dev/null 2>&1
fi
    tar -xvzf ${__0_archive} > /dev/null 2>&1;
    __AS=$?;
if [ $__AS != 0 ]; then
        echo "Please make sure that you have \`tar\` command installed."
        exit__26_v0 1;
        __AF_exit26_v0__125_7=$__AF_exit26_v0;
        echo $__AF_exit26_v0__125_7 > /dev/null 2>&1
fi
    chmod +x "npk";
    __AS=$?;
if [ $__AS != 0 ]; then
        echo "Failed to give permissions to execute npmpink."
        exit__26_v0 1;
        __AF_exit26_v0__131_7=$__AF_exit26_v0;
        echo $__AF_exit26_v0__131_7 > /dev/null 2>&1
fi
    mv ./npk ${bins_folder};
    __AS=$?;
if [ $__AS != 0 ]; then
        echo "failed to move bin to ${bins_folder}"
        exit__26_v0 1;
        __AF_exit26_v0__137_5=$__AF_exit26_v0;
        echo $__AF_exit26_v0__137_5 > /dev/null 2>&1
fi
    echo "Installed to ${bins_folder}/npk"
    ${bins_folder}/npk --version;
    __AS=$?;
if [ $__AS != 0 ]; then
        echo "installing seems have failed"
        exit__26_v0 1;
        __AF_exit26_v0__144_5=$__AF_exit26_v0;
        echo $__AF_exit26_v0__144_5 > /dev/null 2>&1
fi
    echo "Make sure '${bins_folder}' is inside your PATH"
    echo "To upgrade, run this script again"
    echo ""
    echo "run \`npk --help\` to see usage"