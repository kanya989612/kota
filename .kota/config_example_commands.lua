-- Example configuration with custom commands
kota.setup({
  model = "gpt-4o",
  api_key = os.getenv("OPENAI_API_KEY") or "",
  api_base = "https://api.openai.com/v1",
  temperature = 0.7,
  
  -- Tool configuration
  tools = {
    enabled = { "grep", "write_file", "read_file", "edit_file" },
    disabled = {},
  },
  
  -- Custom commands with parameter support
  commands = {
    -- Simple string command
    ["fix"] = "analyze and fix the current file",
    
    -- Function command with named parameters
    ["test"] = function(args)
      local file = args.file or "current file"
      return "run tests for " .. file
    end,
    
    -- Function command with multiple parameters
    ["review"] = function(args)
      local file = args.file or "current file"
      local aspect = args.aspect or "general quality"
      return string.format("review %s focusing on %s", file, aspect)
    end,
    
    -- Function command supporting both named and positional args
    ["refactor"] = function(args)
      local file = args.file or args["1"] or "current file"
      local pattern = args.pattern or args["2"] or "improve code structure"
      return string.format("refactor %s to %s", file, pattern)
    end,
    
    -- Function command with default values
    ["doc"] = function(args)
      local file = args.file or args["1"] or "current file"
      local style = args.style or "markdown"
      return string.format("generate %s documentation for %s", style, file)
    end,
    
    -- Function command for code analysis
    ["analyze"] = function(args)
      local target = args.target or args["1"] or "current file"
      local depth = args.depth or "detailed"
      return string.format("perform %s analysis on %s", depth, target)
    end,
  },
})
