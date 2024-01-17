Name:
Version:
Release: 1%{?dist}
Summary: Collection of GitHub user dboeger1's personal configuration files.

License: None
#URL:
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
