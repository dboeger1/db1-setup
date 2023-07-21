--[[

    FZF Finder for Telescope


    References:

        :help telescope-fzf-native.nvim
        :help telescope.nvim

        https://github.com/nvim-telescope/telescope-fzf-native.nvim
        https://github.com/nvim-telescope/telescope.nvim

--]]


return {
    'nvim-telescope/telescope-fzf-native.nvim',
    enabled = true,
    cond = function()
        return vim.fn.executable('make') == 1
    end,
    build = 'make',
}
