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
    exec install -D -t %{buildroot}/usr/bin/tmux \
        .tmux.conf
)

(
    cd neovim &&
    exec install -D -t %{buildroot}/usr/bin/neovim \
        init.lua \
        lazy-lock.json
)

(
    cd neovim/lua &&
    exec install -D -t %{buildroot}/usr/bin/neovim/lua \
        .luarc.json
)

(
    cd neovim/lua/plugins &&
    exec install -D -t %{buildroot}/usr/bin/neovim/lua/plugins \
        plugin_netrw.lua
)

(
    cd neovim/lua/plugins/lazy &&
    exec install -D -t %{buildroot}/usr/bin/neovim/lua/plugins/lazy \
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
    exec install -D -t %{buildroot}/usr/bin/neovim/lua/settings \
        indentation.lua \
        information.lua \
        key_maps.lua \
        tabs.lua \
        themes.lua \
        windows.lua
)


%check


%files
/usr/bin/tmux/.tmux.conf
/usr/bin/neovim/init.lua
/usr/bin/neovim/lazy-lock.json
/usr/bin/neovim/lua/.luarc.json
/usr/bin/neovim/lua/plugins/plugin_netrw.lua
/usr/bin/neovim/lua/plugins/lazy/plugin_cmp.lua
/usr/bin/neovim/lua/plugins/lazy/plugin_kanagawa.lua
/usr/bin/neovim/lua/plugins/lazy/plugin_lualine.lua
/usr/bin/neovim/lua/plugins/lazy/plugin_mason.lua
/usr/bin/neovim/lua/plugins/lazy/plugin_nightfox.lua
/usr/bin/neovim/lua/plugins/lazy/plugin_snippy.lua
/usr/bin/neovim/lua/plugins/lazy/plugin_telescope.lua
/usr/bin/neovim/lua/plugins/lazy/plugin_telescope_fzf_native.lua
/usr/bin/neovim/lua/plugins/lazy/plugin_treesitter.lua
/usr/bin/neovim/lua/settings/indentation.lua
/usr/bin/neovim/lua/settings/information.lua
/usr/bin/neovim/lua/settings/key_maps.lua
/usr/bin/neovim/lua/settings/tabs.lua
/usr/bin/neovim/lua/settings/themes.lua
/usr/bin/neovim/lua/settings/windows.lua


%changelog
