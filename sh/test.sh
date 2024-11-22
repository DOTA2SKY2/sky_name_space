#!/bin/bash
################################文件头############################
# Copyright 2024, GRT Co. Ltd.
# All rights reserved.
# FileName:    test.sh
# Description: first case for test.
# Author:      Sky
# http://www.xxxxxx.com
# Revision: 1.0.0
#################################################################

#set -e            	#打开异常退出功能
 set -x            #打开Debug功能

######################定义变量######################
source /etc/profile		#避免用contab、ansible、Jenkins执行shell脚本出现环境变量不对的问题
WORKDIR="$(cd $(dirname "$0") || exit 1;pwd)"		#脚本所在路径
echo "Current Excute: bash ${WORKDIR}/$0 $@"

function_dir=${WORKDIR}/myfunction.lib

#创建日志文件
if [[ ! -d ${WORKDIR}/logs ]];then
    mkdir -p ${WORKDIR}/logs
else
    rm -f ${WORKDIR}/logs/*.log
fi
[[ $UID -ne 0 ]] && echo "[ERROR] Please Use root Excute......" && exit 1

#输出信息
# ${FUNCNAME[0]代表当前函数名，$LINENO代表当前代码行号
echo "$(date "+%Y-%m-%d %T.%N")[ERROR ${FUNCNAME[0]}:$LINENO] Invalid Param"
echo "$(date "+%Y-%m-%d %T.%N")[INFO]:Install Success"



#加载函数库
if [[ -f "$function_dir" ]];then
 	source $function_dir
else
	echo -e "\033[31m函数库不存在\033[0m"
	exit 71
fi




######################功能函数######################
#检查环境
Check_Env() {
    echo "[INFO] Begin Check_Env..."
    [[ $UID -ne 0 ]] && echo "[ERROR] Please Use Admin(root) Excute......" && exit 1
    #检查命令是否存在
    for cmd_bin in curl mvn
    do
        if ! command -v ${cmd_bin} &> /dev/null;then
            echo "[ERROR] ${cmd_bin} command Not Exist" && exit 1
        fi
    done
    echo "[INFO] Check_Env Success"
}



#帮助信息
Help() {
	cat << EOF
Usage:
=======================================================================
optional arguments:
	-h	提供帮助信息
	-num	虚拟机编号
EXAMPLE:
	bash $0 -num 10 web1 eth0 192.168.4.1/24
EOF
}



#######################主函数#######################
#参数校验
[[ $# -ne 1 ]] && echo "[ERROR] Invalid Param!!! eg:bash $0 ansible_path" && exit 1
[[ $# -le 5 ]] && echo "[ERROR] Invalid Param!!!,Please Excute:bash $0 -h" && exit 1


#主函数
main() {
    Print
    cecho 32 "开始执行......"
    echo "1.本地源"
    echo "2.网络源"
    read -p  "请选择:" choice
    case ${choice} in
    1)
        Conf_Apt;;
    2)
        echo "1.ubuntu14.04"
        echo "2.ubuntu16.04"
        echo "3.ubuntu18.04"
        read -p "请选择系统版本:" choice
        case ${choice} in
        1)
            Conf_Ubuntu14.04;;
        2)
            Conf_Ubuntu16.04;;
        3)
            Conf_Ubuntu18.04;;
        *)
            cecho 31 "Invalid option!"
        esac
        ;;
    *)
        cecho 31 "Invalid option!"
    esac
}

main

if [[ $# -eq 0 ]];then
    Excute_All
elif [[ "$1" == "-c" -a "$#" -eq 2 ]];then
    case $2 in
    system)
        Init_System;;
    *)
        cecho 31 "Invalid option:bash `basename $0` [-h]"
    esac
elif [[ "$1" == "-h" ]];then
    Help
else
    Help && exit 1
fi

#读取短参数
[[ $# -eq 0 ]] && HELP
while getopts :hnum::a: ARGS
do
	case $ARGS in
	h)
		HELP;;
	nu|m)
		Name=rh7_node$OPTARG;;
	\?)
		cecho 31 "Invalid option:bash `basename $0` [-h]"
	esac
done

