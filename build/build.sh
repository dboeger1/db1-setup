#!/usr/bin/bash


#
# Basic program information.
#
PROGRAM="$(basename ${0})"
PWD="$(pwd)"


#
# Options.
#
OPTSTR=""

CHAR_OPT_HELP="h"
opt_help="false"
OPTSTR+="${CHAR_OPT_HELP}"

CHAR_OPT_USAGE="u"
opt_usage="false"
OPTSTR+="${CHAR_OPT_USAGE}"

CHAR_OPT_BUILD="b"
opt_build="false"
STR_ARG_BUILD="ARTIFACT"
STR_ARG_BUILD_SRC="src"
STR_ARG_BUILD_DEB="deb"
STR_ARG_BUILD_RPM="rpm"
STR_ARG_BUILD_ALL="all"
DEFAULT_ARG_BUILD=""
arg_build="${DEFAULT_ARG_BUILD}"
OPTSTR+="${CHAR_OPT_BUILD}:"

CHAR_OPT_CLEAN="c"
opt_clean="false"
STR_ARG_CLEAN="ARTIFACT"
STR_ARG_CLEAN_SRC="src"
STR_ARG_CLEAN_DEB="deb"
STR_ARG_CLEAN_RPM="rpm"
STR_ARG_CLEAN_ALL="all"
DEFAULT_ARG_CLEAN=""
arg_clean="${DEFAULT_ARG_CLEAN}"
OPTSTR+="${CHAR_OPT_CLEAN}:"

CHAR_OPT_FORCE="f"
opt_force="false"
OPTSTR+="${CHAR_OPT_FORCE}"

CHAR_OPT_VERBOSE="v"
opt_verbose="false"
OPTSTR+="${CHAR_OPT_VERBOSE}"

CHAR_OPT_SRCDIR="S"
opt_srcdir="false"
STR_ARG_SRCDIR="SRC_DIR"
DEFAULT_ARG_SRCDIR="${PWD}"
arg_srcdir="${DEFAULT_ARG_SRCDIR}"
OPTSTR+="${CHAR_OPT_SRCDIR}:"

CHAR_OPT_DESTDIR="D"
opt_destdir="false"
STR_ARG_DESTDIR="DEST_DIR"
DEFAULT_ARG_DESTDIR="${PWD}"
arg_destdir="${DEFAULT_ARG_DESTDIR}"
OPTSTR+="${CHAR_OPT_DESTDIR}:"

CHAR_OPT_VERSION="V"
opt_version="false"
STR_ARG_VERSION="VERSION"
DEFAULT_ARG_VERSION="0.0.1"
arg_version="${DEFAULT_ARG_VERSION}"
OPTSTR+="${CHAR_OPT_VERSION}:"


#
# Usage statement.
#
USAGE="$(cat << END
Usage: %s {-%c | -%c | -%c %s | -%c %s} [-%c%c]
        [-%c %s] [-%c %s] [-%c %s]
    Run \"%s -%c\" for more help.
END
)"
USAGE=$(printf "${USAGE}"   \
    "${PROGRAM}"            \
    "${CHAR_OPT_HELP}"      \
    "${CHAR_OPT_USAGE}"     \
    "${CHAR_OPT_BUILD}"     \
    "${STR_ARG_BUILD}"      \
    "${CHAR_OPT_CLEAN}"     \
    "${STR_ARG_CLEAN}"      \
    "${CHAR_OPT_FORCE}"     \
    "${CHAR_OPT_VERBOSE}"   \
    "${CHAR_OPT_SRCDIR}"    \
    "${STR_ARG_SRCDIR}"     \
    "${CHAR_OPT_DESTDIR}"   \
    "${STR_ARG_DESTDIR}"    \
    "${CHAR_OPT_VERSION}"   \
    "${PROGRAM}"            \
    "${CHAR_OPT_HELP}"      \
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
HELP="$(cat << END
${USAGE}

Help options:

    -%c              Display this help message. This option overrides all other
                    options.

    -%c              Display a brief usage statement. This option overrides all
                    other options except for -%c.

Mode options:

    -%c %s     Build the specified artifacts. Artifacts are built in the
                    destination directory from sources in the source directory.
                    By default, the current working directory is treated as both
                    the source and destination directory, but alternative values
                    may be specified with -%c and -%c, respectively. The
                    %s argument specifies which artifacts to build. Valid
                    values are "%s", "%s", "%s", and "%s". "%s" produces a
                    bundle of the sources. "%s" and "%s" produce a package of
                    the respective format from the source bundle. As such, if
                    the source bundle is not found, both "%s" and "%s" first
                    perform the "%s" build. By extension, "%s" produces all of
                    the previously mentioned artifacts. If multiple instances of
                    this option are provided, all specified artifacts are built.

    -%c %s     Clean the specified build artifacts from the destination
                    directory. By default, the current working directory is
                    treated as the destination directory, but an alternative
                    value may be specified with -%c. The %s argument
                    specifies which artifacts to clean. Valid values are "%s",
                    "%s", "%s", and "%s". "%s" removes the source bundle.
                    "%s" and "%s" remove the respective package. "%s" removes
                    all of the previously mentioned artifacts. If multiple
                    instances of this option are provided, all specified
                    artifacts are removed. If none of the build artifacts remain
                    in the destination directory after performing all clean
                    operations, the destination directory is also removed.

Common mode modifier options:

    -%c %s      Specify an alternative source directory containing sources
                    to be built. By default, the current working directory is
                    treated as the source directory.

    -%c %s     Specify an alternative destination directory for build
                    artifacts. By default, the current working directory is
                    treated as the destination directory.

    -%c %s      Specify the version string of the sources being built. By
                    default, "%s" is used.

Build mode modifier options:

    -%c              Force rebuilding and overwriting of existing artifacts in
                    the destination directory. Without this option, -%c does not
                    overwrite existing artifact files.

Informational options:

    -%c              Enable verbose output.
END
)"
HELP=$(printf "${HELP}"         \
    "${CHAR_OPT_HELP}"          \
    "${CHAR_OPT_USAGE}"         \
    "${CHAR_OPT_HELP}"          \
    "${CHAR_OPT_BUILD}"         \
    "${STR_ARG_BUILD}"          \
    "${CHAR_OPT_SRCDIR}"        \
    "${CHAR_OPT_DESTDIR}"       \
    "${STR_ARG_BUILD}"          \
    "${STR_ARG_BUILD_SRC}"      \
    "${STR_ARG_BUILD_DEB}"      \
    "${STR_ARG_BUILD_RPM}"      \
    "${STR_ARG_BUILD_ALL}"      \
    "${STR_ARG_BUILD_SRC}"      \
    "${STR_ARG_BUILD_DEB}"      \
    "${STR_ARG_BUILD_RPM}"      \
    "${STR_ARG_BUILD_DEB}"      \
    "${STR_ARG_BUILD_RPM}"      \
    "${STR_ARG_BUILD_SRC}"      \
    "${STR_ARG_BUILD_ALL}"      \
    "${CHAR_OPT_CLEAN}"         \
    "${STR_ARG_CLEAN}"          \
    "${CHAR_OPT_DESTDIR}"       \
    "${STR_ARG_CLEAN}"          \
    "${STR_ARG_CLEAN_SRC}"      \
    "${STR_ARG_CLEAN_DEB}"      \
    "${STR_ARG_CLEAN_RPM}"      \
    "${STR_ARG_CLEAN_ALL}"      \
    "${STR_ARG_CLEAN_SRC}"      \
    "${STR_ARG_CLEAN_DEB}"      \
    "${STR_ARG_CLEAN_RPM}"      \
    "${STR_ARG_CLEAN_ALL}"      \
    "${CHAR_OPT_SRCDIR}"        \
    "${STR_ARG_SRCDIR}"         \
    "${CHAR_OPT_DESTDIR}"       \
    "${STR_ARG_DESTDIR}"        \
    "${CHAR_OPT_VERSION}"       \
    "${STR_ARG_VERSION}"        \
    "${DEFAULT_ARG_VERSION}"    \
    "${CHAR_OPT_FORCE}"         \
    "${CHAR_OPT_BUILD}"         \
    "${CHAR_OPT_VERBOSE}"       \
)

exit_help()
{
    echo "${HELP}"
    exit 0
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
        opt_build="true"
        arg_build="${OPTARG}"
        ;;
    "${CHAR_OPT_CLEAN}")
        opt_clean="true"
        arg_clean="${OPTARG}"
        ;;
    "${CHAR_OPT_FORCE}")
        opt_force="true"
        ;;
    "${CHAR_OPT_VERBOSE}")
        opt_verbose="true"
        ;;
    "${CHAR_OPT_SRCDIR}")
        opt_srcdir="true"
        arg_srcdir="${OPTARG}"
        ;;
    "${CHAR_OPT_DESTDIR}")
        opt_destdir="true"
        arg_destdir="${OPTARG}"
        ;;
    "${CHAR_OPT_VERSION}")
        opt_version="true"
        arg_version="${OPTARG}"
        ;;
    "?")
        exit_usage_err
        ;;
    esac
done


#
# Validate and perform preemptive tasks.
#
if
    [ "${opt_help}"  != "true" ] &&
    [ "${opt_usage}" != "true" ] &&
    [ "${opt_build}" != "true" ] &&
    [ "${opt_clean}" != "true" ]
then
    printf "Must specify at least one: {-%c, -%c, -%c, -%c}\n" \
        "${CHAR_OPT_HELP}"  \
        "${CHAR_OPT_USAGE}" \
        "${CHAR_OPT_BUILD}" \
        "${CHAR_OPT_CLEAN}" 1>&2
    exit_usage_err
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
if [ "${opt_build}" = "true" ] && [ "${opt_clean}" = "true" ]
then
    echo "-${CHAR_OPT_BUILD} and -${CHAR_OPT_CLEAN} are mutually exclusive." \
        1>&2
    exit_usage_err
fi

if
    [ "${opt_build}" = "true" ]     &&
    [ "${arg_build}" != "src" ] &&
    [ "${arg_build}" != "rpm" ] &&
    [ "${arg_build}" != "deb" ] &&
    [ "${arg_build}" != "all" ]
then
    echo "Invalid -${CHAR_OPT_BUILD} argument: \"${arg_build}\"" 1>&2
    echo "Must be one of: {\"src\", \"rpm\", \"deb\", \"all\"}" 1>&2
    exit_usage_err
fi

if
    [ "${opt_clean}" = "true" ]     &&
    [ "${arg_clean}" != "src" ] &&
    [ "${arg_clean}" != "rpm" ] &&
    [ "${arg_clean}" != "deb" ] &&
    [ "${arg_clean}" != "all" ]
then
    echo "Invalid -${CHAR_OPT_CLEAN} argument: \"${arg_clean}\"" 1>&2
    echo "Must be one of: {\"src\", \"rpm\", \"deb\", \"all\"}" 1>&2
    exit_usage_err
fi

if [ ! -d "${arg_srcdir}" ]
then
    echo "Source directory does not exist: \"${arg_srcdir}\"" 1>&2
    exit_usage_err
fi

if [ ! -d "${arg_destdir}" ]
then
    echo "Destination directory does not exist: \"${arg_destdir}\"" 1>&2
    exit_usage_err
fi

# TODO: Validate arg_version.


#
# Build.
#
if [ "${opt_build}" = "true" ]
then
    echo "Building..."
    exit 0
    #tar -c -C "${opt_srcdir}" -f "${opt_file}" dboeger1-dotfiles-0.0.1
fi

#
# Clean.
#
if [ "${opt_clean}" = "true" ]
then
    echo "Cleaning..."
    exit 0
fi
