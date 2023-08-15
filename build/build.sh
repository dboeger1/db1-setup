#!/bin/sh


#
# Basic information and constants.
#
PROGRAM_REL_PATH="${0}"
PROGRAM_REL_DIR="$(dirname ${PROGRAM_REL_PATH})"

PROGRAM_NAME="$(basename ${PROGRAM_REL_PATH})"
PROGRAM_DIR="$(cd ${PROGRAM_REL_DIR} && pwd)"
PROGRAM_PATH="${PROGRAM_DIR}/${PROGRAM_NAME}"

PROJECT_ROOT_DIR="$(dirname ${PROGRAM_DIR})"
PROJECT_SRC_DIR="${PROJECT_ROOT_DIR}/src"
PROJECT_BUILD_DIR="${PROJECT_ROOT_DIR}/build"

BUILD_SRC_DIR="${PROJECT_BUILD_DIR}/src"
BUILD_DEB_DIR="${PROJECT_BUILD_DIR}/deb"
BUILD_RPM_DIR="${PROJECT_BUILD_DIR}/rpm"

DPKG_DIR="${BUILD_DEB_DIR}/DEB"

RPMBUILD_DIR="${BUILD_RPM_DIR}/rpmbuild"
RPMBUILD_BUILD_DIR="${RPMBUILD_DIR}/BUILD"
RPMBUILD_BUILDROOT_DIR="${RPMBUILD_DIR}/BUILDROOT"
RPMBUILD_RPMS_DIR="${RPMBUILD_DIR}/RPMS"
RPMBUILD_SOURCES_DIR="${RPMBUILD_DIR}/SOURCES"
RPMBUILD_SPECS_DIR="${RPMBUILD_DIR}/SPECS"
RPMBUILD_SRPMS_DIR="${RPMBUILD_DIR}/SRPMS"

DEB_CONTROL_FILE_NAME="control"
DEB_CONTROL_FILE_PATH="${BUILD_DEB_DIR}/${DEB_CONTROL_FILE_NAME}"
DPKG_CONTROL_FILE_PATH="${DPKG_DIR}/${DEB_CONTROL_FILE_NAME}"

RPM_SPEC_FILE_NAME="name.spec"
RPM_SPEC_FILE_PATH="${BUILD_RPM_DIR}/${RPM_SPEC_FILE_NAME}"
RPMBUILD_SPEC_FILE_PATH="${RPMBUILD_SPECS_DIR}/${RPM_SPEC_FILE_NAME}"


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
OPTSTR+="${CHAR_OPT_BUILD}"

CHAR_OPT_CLEAN="c"
opt_clean="false"
OPTSTR+="${CHAR_OPT_CLEAN}"

CHAR_OPT_FORCE="f"
opt_force="false"
OPTSTR+="${CHAR_OPT_FORCE}"

CHAR_OPT_VERSION="V"
opt_version="false"
STR_ARG_VERSION="VERSION"
DEFAULT_ARG_VERSION="0.0.1"
arg_version="${DEFAULT_ARG_VERSION}"
OPTSTR+="${CHAR_OPT_VERSION}:"


#
# Help messages.
#
USAGE=$(printf \
    "Usage: %s {-%c|-%c|-%c|-%c} [-%c] [-%c %s]" \
    "${PROGRAM_NAME}"       \
    "${CHAR_OPT_HELP}"      \
    "${CHAR_OPT_USAGE}"     \
    "${CHAR_OPT_BUILD}"     \
    "${CHAR_OPT_CLEAN}"     \
    "${CHAR_OPT_FORCE}"     \
    "${CHAR_OPT_VERSION}"   \
    "${STR_ARG_VERSION}"    \
)

HINT=$(printf \
    "Run \"%s -%c\" for more help." \
    "${PROGRAM_NAME}"   \
    "${CHAR_OPT_HELP}"  \
)

HELP_FORMAT=$(cat << END

Help:
    -%c          Display this help message.
    -%c          Display a brief usage statement.

Actions:
    -%c          Build all artifacts.
    -%c          Clean all artifacts.

Modifiers:
    -%c          Force rebuilding of artifacts.
    -%c %s  Specify version of sources (default="%s").
END
)
HELP=$(printf "${HELP_FORMAT}"  \
    "${CHAR_OPT_HELP}"          \
    "${CHAR_OPT_USAGE}"         \
    "${CHAR_OPT_BUILD}"         \
    "${CHAR_OPT_CLEAN}"         \
    "${CHAR_OPT_FORCE}"         \
    "${CHAR_OPT_VERSION}"       \
    "${STR_ARG_VERSION}"        \
    "${DEFAULT_ARG_VERSION}"    \
)

exit_usage()
{
    printf "${USAGE}\n${HINT}\n"
    exit 0
}

exit_usage_err()
{
    printf "${USAGE}\n${HINT}\n" 1>&2
    exit 1
}

exit_help()
{
    printf "${USAGE}\n${HELP}\n"
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
        ;;
    "${CHAR_OPT_CLEAN}")
        opt_clean="true"
        ;;
    "${CHAR_OPT_FORCE}")
        opt_force="true"
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
    printf "-%c and -%c are mutually exclusive.\n" \
        "${CHAR_OPT_BUILD}" \
        "${CHAR_OPT_CLEAN}" 1>&2
    exit_usage_err
fi

# TODO: Validate arg_version.
src_name="${arg_name}-${arg_version}"
src_file_name="${src_name}.tar.gz"
src_file_path="${BUILD_SRC_DIR}/${src_file}"

dpkg_src_file_path="${DPKG_DIR}/${src_file_name}"

rpmbuild_src_file_path="${RPMBUILD_SOURCES_DIR}/${src_file_name}"


#
# Build.
#
if [ "${opt_build}" = "true" ]
then
    # src
    mkdir -p ${BUILD_SRC_DIR}
    tar -c -C "${PROJECT_SRC_DIR}" -f "${src_file_path}" \
        --transform="s#^#${src_name}/#" \
        neovim tmux

    # deb
    mkdir -p ${DPKG_DIR}
    cp ${src_file_path} ${dpkg_src_file_path}
    # TODO: Replace following line with actual file modification and packaging.
    cp ${DEB_CONTROL_FILE_PATH} ${DPKG_CONTROL_FILE_PATH}

    # rpm
    mkdir -p \
        ${RPMBUILD_BUILD_DIR}       \
        ${RPMBUILD_BUILDROOT_DIR}   \
        ${RPMBUILD_RPMS_DIR}        \
        ${RPMBUILD_SOURCES_DIR}     \
        ${RPMBUILD_SPECS_DIR}       \
        ${RPMBUILD_SRPMS_DIR}
    cp ${src_file_path} ${rpmbuild_src_file_path}
    sed \
        -e "s#^Name:\$#Name: ${arg_name}#" \
        -e "s#^Version:\$#Version: ${arg_version}#" \
        -e "s#^Source0:\$#Source0: ${rpmbuild_src_file_path}#" \
        ${RPM_SPEC_FILE_PATH} > ${RPMBUILD_SPEC_FILE_PATH}
    rpmbuild --define "_topdir ${RPMBUILD_DIR}" -ba "${RPMBUILD_SPEC_FILE_PATH}"

    exit 0
fi

#
# Clean.
#
if [ "${opt_clean}" = "true" ]
then
    for dir in "${RPMBUILD_DIR}" "${DEB_DIR}" "${BUILD_SRC_DIR}"
    do
        if [ -e "${dir}" ]
        then
            if [ ! -d "${dir}" ]
            then
                printf "Cannot clean non-directory: \"%s\"\n" "${dir}" 1>&2
                exit 1
            fi

            rm -rf ${dir}
        fi
    done

    exit 0
fi
