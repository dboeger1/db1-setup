--[[

    Lualine Configurable Status Line


    References:

        :help lualine.txt

        https://github.com/nvim-lualine/lualine.nvim

--]]


return {
    'nvim-lualine/lualine.nvim',
    enabled = true,
    --dependencies = {},
    opts = {
        options = {
            icons_enabled = false,
            component_separators = '_',
            section_separators = '|',
        },
        sections = {
            lualine_a = {'mode'},
            lualine_b = {'location', 'progress'},
            lualine_c = {},
            lualine_x = {},
            lualine_y = {'filetype', 'encoding'},
            lualine_z = {'filename'},
        },
        inactive_sections = {
            lualine_a = {},
            lualine_b = {},
            lualine_c = {},
            lualine_x = {},
            lualine_y = {},
            lualine_z = {'filename'},
        },
    },
}
