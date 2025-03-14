--[[

    Telescope Fuzzy Finder


    References:

        https://github.com/nvim-telescope/telescope.nvim
        https://github.com/nvim-lua/plenary.nvim
        https://github.com/nvim-telescope/telescope-fzf-native.nvim

--]]


-- Key Map
vim.api.nvim_set_keymap('n', '<Leader>f', ':Telescope<CR>', {noremap = true, silent = true})
vim.api.nvim_set_keymap('v', '<Leader>f', ':Telescope<CR>', {noremap = true, silent = true})

-- Lazy Spec
return {
    'nvim-telescope/telescope.nvim',
    enabled = true,
    dependencies = {
        'nvim-lua/plenary.nvim',
        'nvim-telescope/telescope-fzf-native.nvim',
    },
    config = function()
        -- Dependency Modules
        local telescope = require('telescope')

        -- Setup
        telescope.setup({})

        -- Load FZF Extension
        telescope.load_extension('fzf')
    end,
}
