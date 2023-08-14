Name:
Version:
Release: 1%{?dist}
Summary: Collection of GitHub user dboeger1's personal configuration files.

License: None
#URL:
Source0:/root/rpmbuild/SOURCES/dboeger1-dotfiles-0.0.1.tar.gz
#
#BuildRequires:
#Requires:

BuildArch: noarch

%{!?prefix_app: %global prefix_app /opt}
%global app_dir %{prefix_app}/%{name}

%description


%prep
%setup


%build


%install
(
    cd tmux &&
    exec install -D -t %{buildroot}/%{app_dir}/tmux \
        .tmux.conf
)

(
    cd neovim &&
    exec install -D -t %{buildroot}/%{app_dir}/neovim \
        init.lua \
        lazy-lock.json
)

(
    cd neovim/lua &&
    exec install -D -t %{buildroot}/%{app_dir}/neovim/lua \
        .luarc.json
)

(
    cd neovim/lua/plugins &&
    exec install -D -t %{buildroot}/%{app_dir}/neovim/lua/plugins \
        plugin_netrw.lua
)

(
    cd neovim/lua/plugins/lazy &&
    exec install -D -t %{buildroot}/%{app_dir}/neovim/lua/plugins/lazy \
        plugin_cmp.lua \
        plugin_kanagawa.lua \
        plugin_lualine.lua \
        plugin_mason.lua \
        plugin_nightfox.lua \
        plugin_snippy.lua \
        plugin_telescope.lua \
        plugin_telescope_fzf_native.lua \
        plugin_treesitter.lua
)

(
    cd neovim/lua/settings &&
    exec install -D -t %{buildroot}/%{app_dir}/neovim/lua/settings \
        indentation.lua \
        information.lua \
        key_maps.lua \
        tabs.lua \
        themes.lua \
        windows.lua
)


%check


%files
/%{app_dir}/tmux/.tmux.conf
/%{app_dir}/neovim/init.lua
/%{app_dir}/neovim/lazy-lock.json
/%{app_dir}/neovim/lua/.luarc.json
/%{app_dir}/neovim/lua/plugins/plugin_netrw.lua
/%{app_dir}/neovim/lua/plugins/lazy/plugin_cmp.lua
/%{app_dir}/neovim/lua/plugins/lazy/plugin_kanagawa.lua
/%{app_dir}/neovim/lua/plugins/lazy/plugin_lualine.lua
/%{app_dir}/neovim/lua/plugins/lazy/plugin_mason.lua
/%{app_dir}/neovim/lua/plugins/lazy/plugin_nightfox.lua
/%{app_dir}/neovim/lua/plugins/lazy/plugin_snippy.lua
/%{app_dir}/neovim/lua/plugins/lazy/plugin_telescope.lua
/%{app_dir}/neovim/lua/plugins/lazy/plugin_telescope_fzf_native.lua
/%{app_dir}/neovim/lua/plugins/lazy/plugin_treesitter.lua
/%{app_dir}/neovim/lua/settings/indentation.lua
/%{app_dir}/neovim/lua/settings/information.lua
/%{app_dir}/neovim/lua/settings/key_maps.lua
/%{app_dir}/neovim/lua/settings/tabs.lua
/%{app_dir}/neovim/lua/settings/themes.lua
/%{app_dir}/neovim/lua/settings/windows.lua


%changelog
