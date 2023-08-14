%{!?prefix_app: %global prefix_app /opt}
%global app_dir %{prefix_app}/dboeger1-dotfiles

Name: dboeger1-dotfiles
Version: 0.0.1
Release: 1%{?dist}
Summary: Collection of GitHub user dboeger1's personal configuration files.

License: None
#URL:
Source0:/root/rpmbuild/SOURCES/dboeger1-dotfiles-0.0.1.tar.gz
#
#BuildRequires:
#Requires:

BuildArch: noarch

%description


%prep
%setup


%build


%install
(
    cd tmux &&
    exec install -D -t %{buildroot}/%{prefix_app}/tmux \
        .tmux.conf
)

(
    cd neovim &&
    exec install -D -t %{buildroot}/%{prefix_app}/neovim \
        init.lua \
        lazy-lock.json
)

(
    cd neovim/lua &&
    exec install -D -t %{buildroot}/%{prefix_app}/neovim/lua \
        .luarc.json
)

(
    cd neovim/lua/plugins &&
    exec install -D -t %{buildroot}/%{prefix_app}/neovim/lua/plugins \
        plugin_netrw.lua
)

(
    cd neovim/lua/plugins/lazy &&
    exec install -D -t %{buildroot}/%{prefix_app}/neovim/lua/plugins/lazy \
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
    exec install -D -t %{buildroot}/%{prefix_app}/neovim/lua/settings \
        indentation.lua \
        information.lua \
        key_maps.lua \
        tabs.lua \
        themes.lua \
        windows.lua
)


%check


%files
/%{prefix_app}/tmux/.tmux.conf
/%{prefix_app}/neovim/init.lua
/%{prefix_app}/neovim/lazy-lock.json
/%{prefix_app}/neovim/lua/.luarc.json
/%{prefix_app}/neovim/lua/plugins/plugin_netrw.lua
/%{prefix_app}/neovim/lua/plugins/lazy/plugin_cmp.lua
/%{prefix_app}/neovim/lua/plugins/lazy/plugin_kanagawa.lua
/%{prefix_app}/neovim/lua/plugins/lazy/plugin_lualine.lua
/%{prefix_app}/neovim/lua/plugins/lazy/plugin_mason.lua
/%{prefix_app}/neovim/lua/plugins/lazy/plugin_nightfox.lua
/%{prefix_app}/neovim/lua/plugins/lazy/plugin_snippy.lua
/%{prefix_app}/neovim/lua/plugins/lazy/plugin_telescope.lua
/%{prefix_app}/neovim/lua/plugins/lazy/plugin_telescope_fzf_native.lua
/%{prefix_app}/neovim/lua/plugins/lazy/plugin_treesitter.lua
/%{prefix_app}/neovim/lua/settings/indentation.lua
/%{prefix_app}/neovim/lua/settings/information.lua
/%{prefix_app}/neovim/lua/settings/key_maps.lua
/%{prefix_app}/neovim/lua/settings/tabs.lua
/%{prefix_app}/neovim/lua/settings/themes.lua
/%{prefix_app}/neovim/lua/settings/windows.lua


%changelog
