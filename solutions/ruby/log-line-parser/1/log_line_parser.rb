class LogLineParser
  def initialize(line)
    @line = line
    @level = line[/(INFO|WARNING|ERROR)/].downcase
    @message = line.split(': ')[1].strip
  end

  def message
    @message
  end

  def log_level
    @level
  end

  def reformat
    "#{@message} (#{@level})"
  end
end
