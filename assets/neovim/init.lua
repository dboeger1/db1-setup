--[[

    Root Neovim Configuration


    References:

        :help lazy.nvim.txt

        https://github.com/folke/lazy.nvim
        https://github.com/nvim-lua/kickstart.nvim

--]]


-- Leader Key
vim.g.mapleader      = ' '
vim.g.maplocalleader = ' '


-- Bootstrap Lazy Plugin Manager
local lazypath = vim.fn.stdpath 'data' .. '/lazy/lazy.nvim'
if not vim.loop.fs_stat(lazypath) then
    vim.fn.system {
        'git',
        'clone',
        '--filter=blob:none',
        'https://github.com/folke/lazy.nvim.git',
        '--branch=stable', -- latest stable release
        lazypath,
    }
end
vim.opt.rtp:prepend(lazypath)


-- Initialize Lazy
require('lazy').setup(
    { import = 'plugins.lazy' }, -- plugin specs subdirectory
    {}                           -- options
)


-- Options
require('settings.key_maps')
require('settings.windows')
require('settings.tabs')
require('settings.information')
require('settings.indentation')
require('settings.themes')


-- Built-In Plugins
require('plugins.plugin_netrw')
