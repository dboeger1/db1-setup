#!/usr/bin/bash
PS4='${LINENO}: '
set -x

#
# Usage information.
#
USAGE=$(cat << END
Usage: $(basename ${0}) -c -f <filename>
END
)

usage()
{
    echo "${USAGE}"
    exit 0
}

usage_err()
{
    echo "${USAGE}" 1>&2
    exit 1
}


#
# Options.
#
OPTSTR=""

# Help: -h
CHAR_OPT_HELP="h"
OPTSTR+="${CHAR_OPT_HELP}"

# Clean: -c
CHAR_OPT_CLEAN="c"
DEFAULT_OPT_CLEAN="false"
opt_clean="${DEFAULT_OPT_CLEAN}"
OPTSTR+="${CHAR_OPT_CLEAN}"

# Force: -F
CHAR_OPT_FORCE="F"
DEFAULT_OPT_FORCE="false"
opt_force="${DEFAULT_OPT_FORCE}"
OPTSTR+="${CHAR_OPT_FORCE}"

# Source directory: -s
CHAR_OPT_SRCDIR="d"
DEFAULT_OPT_SRCDIR="../../src"
opt_srcdir="${DEFAULT_OPT_SRCDIR}"
OPTSTR+="${CHAR_OPT_SRCDIR}:"

# File: -f <file name>
CHAR_OPT_FILE="f"
DEFAULT_OPT_FILE="dboeger1-dotfiles-0.0.1.tar.gz"
opt_file="${DEFAULT_OPT_FILE}"
OPTSTR+="${CHAR_OPT_FILE}:"


#
# Parse options.
#
while getopts "${OPTSTR}" OPT
do
    case "${OPT}" in
    "${CHAR_OPT_HELP}")
        usage
        ;;
    "${CHAR_OPT_CLEAN}")
        opt_clean="true"
        ;;
    "${CHAR_OPT_FORCE}")
        opt_force="true"
        ;;
    "${CHAR_OPT_SRCDIR}")
        opt_srcdir="${OPTARG}"
        ;;
    "${CHAR_OPT_FILE}")
        opt_file="${OPTARG}"
        ;;
    "?")
        usage_err
        ;;
    esac
done


#
# Validate options.
#

# Clean
if [ "${opt_clean}" != "true" ] && [ "${opt_clean}" != "false" ]
then
    echo "Invalid clean option: \"${opt_clean}\"" 1>&2
    usage_err
fi

# Force
if [ "${opt_force}" != "true" ] && [ "${opt_force}" != "false" ]
then
    echo "Invalid force option: \"${opt_force}\"" 1>&2
    usage_err
fi

# Source Directory
if [ "${opt_clean}" != "true" ] && [ ! -d "${opt_srcdir}" ]
then
    echo "Source directory does not exist: \"${opt_srcdir}\"" 1>&2
    usage_err
fi

# File
if [ -z "${opt_file}" ]
then
    echo "Invalid empty file option: \"${opt_file}\"" 1>&2
    usage_err
fi

if [ "${opt_clean}" = "true" ]
then
    if [ ! -e "${opt_file}" ]
    then
        echo "Cannot clean nonexistent file: \"${opt_file}\"" 1>&2
        usage_err
    fi

    if [ -d "${opt_file}" ]
    then
        echo "Cannot clean directory: \"${opt_file}\"" 1>&2
        usage_err
    fi

    if [ ! -f "${opt_file}" ]
    then
        echo "Cannot clean non-regular file: \"${opt_file}\"" 1>&2
        usage_err
    fi
elif [ -e "${opt_file}" ]
then
    if [ -d "${opt_file}" ]
    then
        echo "Cannot overwrite directory: \"${opt_file}\"" 1>&2
        usage_err
    fi

    if [ ! -f "${opt_file}" ]
    then
        echo "Cannot overwrite non-regular file: \"${opt_file}\"" 1>&2
        usage_err
    fi

    if [ "${opt_force}" = "false" ]
    then
        echo "Must use -F to overwrite existing file: \"${opt_file}\"" 1>&2
        usage_err
    fi
else
    touch "${opt_file}"
    if [ "${?}" -ne "0" ]
    then
        echo "Unable to create file: \"${opt_file}\"" 1>&2
        usage_err
    fi
    rm -f "${opt_file}"
fi


#
# Clean.
#
if [ "${opt_clean}" = "true" ]
then
    rm -f "${opt_file}"
    if [ "${?}" -ne "0" ]
    then
        echo "Failed to delete file: \"${opt_file}\"" 1>&2
        exit 1
    fi

    exit 0
fi


#
# Build.
#

# TODO: Properly handle versioning, root directory, etc.
tar -c -C "${opt_srcdir}" -f "${opt_file}" dboeger1-dotfiles-0.0.1
if [ "${?}" -ne "0" ]
then
    echo "Failed to build archive: \"${opt_file}\"" 1>&2
    exit 1
fi
