Name: dboeger1-dotfiles
Version: 0.1.0
Release: 1%{?dist}
Summary: Collection of GitHub user dboeger1's personal configuration files

License: MIT
URL: https://github.com/dboeger1/%{name}
Source0: 
#
#BuildRequires:
#Requires:

BuildArch:

%global app_destination_dir /opt/%{name}
%global app_source_dir %{buildroot}/%{app_destination_dir}

%description


%prep
%setup


%build
cargo build


%install


%check


%files


%changelog
