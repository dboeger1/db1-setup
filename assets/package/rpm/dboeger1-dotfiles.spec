Name: dboeger1-dotfiles
Version: 0.1.0
Release: 1%{?dist}
Summary: Collection of GitHub user dboeger1's personal configuration files

License: MIT
URL: https://github.com/dboeger1/%{name}
Source0: %{name}-%{version}.tar.gz

BuildArch: %{_target_cpu}

%global dir_install_root /opt/%{name}
%global dir_source_buildroot %{buildroot}/%{dir_install_root}

%description


%prep
%setup


%build
cargo build


%install
install -d %{dir_source_buildroot}
install -D -t %{dir_source_buildroot}/ LICENSE
install -D -t %{dir_source_buildroot}/ target/debug/dbd_configure
cp -r assets/incus %{dir_source_buildroot}/
cp -r assets/neovim %{dir_source_buildroot}/
cp -r assets/tmux %{dir_source_buildroot}/


%files
%dir %{dir_install_root}
%{dir_install_root}/
