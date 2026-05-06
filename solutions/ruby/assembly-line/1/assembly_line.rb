class AssemblyLine
  def initialize(speed)
    @speed = speed
  end

  def production_rate_per_hour
    rate = 0.77
    if @speed <= 4
      rate = 1
    elsif @speed <= 8
      rate = 0.9
    elsif @speed < 10
      rate = 0.8
    end

    @speed * 221 * rate
  end

  def working_items_per_minute
    (production_rate_per_hour /  60).floor
  end
end
