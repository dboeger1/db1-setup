#!/usr/bin/bash


#
# Basic program information.
#
PROGRAM="$(basename ${0})"
VERSION="0.0.1"
INFO="${PROGRAM} v${VERSION}"
PWD="$(pwd)"


#
# Options.
#
OPTSTR=""

CHAR_OPT_HELP="h"
DEFAULT_OPT_HELP="false"
opt_help="${DEFAULT_OPT_HELP}"
OPTSTR+="${CHAR_OPT_HELP}"

CHAR_OPT_USAGE="u"
DEFAULT_OPT_USAGE="false"
opt_usage="${DEFAULT_OPT_USAGE}"
OPTSTR+="${CHAR_OPT_USAGE}"

CHAR_OPT_BUILD="b"
DEFAULT_OPT_BUILD=""
opt_build="${DEFAULT_OPT_BUILD}"
OPTSTR+="${CHAR_OPT_BUILD}:"

CHAR_OPT_CLEAN="c"
DEFAULT_OPT_CLEAN=""
opt_clean="${DEFAULT_OPT_CLEAN}"
OPTSTR+="${CHAR_OPT_CLEAN}:"

CHAR_OPT_FORCE="f"
DEFAULT_OPT_FORCE="false"
opt_force="${DEFAULT_OPT_FORCE}"
OPTSTR+="${CHAR_OPT_FORCE}"

CHAR_OPT_INFO="i"
DEFAULT_OPT_INFO="false"
opt_info="${DEFAULT_OPT_INFO}"
OPTSTR+="${CHAR_OPT_INFO}"

CHAR_OPT_VERBOSE="v"
DEFAULT_OPT_VERBOSE="false"
opt_verbose="${DEFAULT_OPT_VERBOSE}"
OPTSTR+="${CHAR_OPT_VERBOSE}"

CHAR_OPT_SRCDIR="S"
DEFAULT_OPT_SRCDIR="${PWD}"
opt_srcdir="${DEFAULT_OPT_SRCDIR}"
OPTSTR+="${CHAR_OPT_SRCDIR}:"

CHAR_OPT_DESTDIR="D"
DEFAULT_OPT_DESTDIR="${PWD}"
opt_destdir="${DEFAULT_OPT_DESTDIR}"
OPTSTR+="${CHAR_OPT_DESTDIR}:"

CHAR_OPT_VERSION="V"
DEFAULT_OPT_VERSION="0.0.1"
opt_version="${DEFAULT_OPT_VERSION}"
OPTSTR+="${CHAR_OPT_VERSION}:"


#
# Usage statement.
#
USAGE=$(cat << END
Usage: ${PROGRAM} [-hu] [-b ARTIFACT | -c ARTIFACT] [-fiv]
           [-S SRC_DIR] [-D DEST_DIR] [-V VERSION]
END
)

exit_usage()
{
    echo "${USAGE}"
    exit 0
}

exit_usage_err()
{
    echo "${USAGE}" 1>&2
    exit 1
}


#
# Help message.
#
HELP=$(cat << END
${USAGE}

Help options:

    -h              Display this help message, which includes the usage
                    statement displayed by -u. This option overrides all other
                    options except for -i. When none of -h, -u, -b, and -c are
                    specified, this is the default behavior.

    -u              Display a brief usage statement. This option overrides all
                    other options except for -h and -i. This option is
                    overridden by -h, which includes the usage statement in its
                    help message.

Mode options:

    -b ARTIFACT     Build the specified artifacts. Artifacts are built in the
                    destination directory from sources in the source directory.
                    By default, the current working directory is treated as both
                    the source and destination directory, but alternative values
                    may be specified with -S and -D, respectively. The ARTIFACT
                    argument specifies which artifacts to build. Valid values
                    are "src", "rpm", "deb", and "all". "src" produces a bundle
                    of the sources. "rpm" and "deb" produce a package of the
                    respective format from the source bundle. As such, if the
                    source bundle is not found, both "rpm" and "deb" first
                    perform the "src" build. By extension, "all" produces all of
                    the previously mentioned artifacts. If multiple instances of
                    this option are provided, all specified artifacts are built.

    -c ARTIFACT     Clean the specified build artifacts from the destination
                    directory. By default, the current working directory is
                    treated as the destination directory, but an alternative
                    value may be specified with -D. The ARTIFACT argument
                    specifies which artifacts to clean. Valid values are "src",
                    "rpm", "deb", and "all". "src" removes the source bundle.
                    "rpm" and "deb" remove the respective package. "all" removes
                    all of the previously mentioned artifacts. If multiple
                    instances of this option are provided, all specified
                    artifacts are removed. If none of the build artifacts remain
                    in the destination directory after performing all clean
                    operations, the destination directory is also removed.

Common mode modifier options:

    -S SRC_DIR      Specify an alternative source directory containing sources
                    to be built. By default, the current working directory is
                    treated as the source directory.

    -D DEST_DIR     Specify an alternative destination directory for build
                    artifacts. By default, the current working directory is
                    treated as the destination directory.

    -V VERSION      Specify the version string of the sources being built. By
                    default, "${DEFAULT_OPT_VERSION}" is used.

Build mode modifier options:

    -f              Force rebuilding and overwriting of existing artifacts in
                    the destination directory. Without this option, -b does not
                    overwrite existing artifact files.

Informational options:

    -i              Display program information, including version.

    -v              Enable verbose output.
END
)

exit_help()
{
    echo "${HELP}"
    exit 0
}

exit_help_err()
{
    echo "${HELP}" 1>&2
    exit 1
}


#
# Parse options.
#
while getopts "${OPTSTR}" OPT
do
    case "${OPT}" in
    "${CHAR_OPT_HELP}")
        opt_help="true"
        ;;
    "${CHAR_OPT_USAGE}")
        opt_usage="true"
        ;;
    "${CHAR_OPT_BUILD}")
        opt_build="${OPTARG}"
        ;;
    "${CHAR_OPT_CLEAN}")
        opt_clean="${OPTARG}"
        ;;
    "${CHAR_OPT_FORCE}")
        opt_force="true"
        ;;
    "${CHAR_OPT_INFO}")
        opt_info="true"
        ;;
    "${CHAR_OPT_VERBOSE}")
        opt_verbose="true"
        ;;
    "${CHAR_OPT_SRCDIR}")
        opt_srcdir="${OPTARG}"
        ;;
    "${CHAR_OPT_DESTDIR}")
        opt_destdir="${OPTARG}"
        ;;
    "?")
        exit_usage_err
        ;;
    esac
done


#
# Perform preemptive tasks prior to validation of remaining options.
#
if [ "${opt_info}" = "true" ]
then
    echo "${INFO}"
fi

if \
    [ "${opt_help}" != "true" ] && \
    [ "${opt_usage}" != "true" ] && \
    [ "${opt_build}" != "true" ] && \
    [ "${opt_clean}" != "true" ]
then
fi
if [ "${opt_help}" = "true" ]
then
    exit_help
fi

if [ "${opt_usage}" = "true" ]
then
    exit_usage
fi


#
# Validate remaining options.
#
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
