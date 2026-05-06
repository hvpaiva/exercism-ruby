class AssemblyLine
  ITEMS_PER_HOUR = 221

  def initialize(speed)
    @speed = speed
  end

  def production_rate_per_hour
    @speed * ITEMS_PER_HOUR * success_rate
  end

  def working_items_per_minute
    (production_rate_per_hour /  60).floor
  end

  def success_rate
    case @speed
    when 1..4 then 1.0
    when 4..8 then 0.9
    when 9    then 0.8
    else           0.77
    end
  end
end
