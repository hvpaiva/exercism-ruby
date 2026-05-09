class ResistorColorDuo
  BANDS = %w[black brown red orange yellow green blue violet grey white]
    .each_with_index.to_h.freeze

  class InvalidColor < StandardError; end

  def self.value(colors)
    first, second = colors

    raise InvalidColor unless BANDS.key?(first) && BANDS.key?(second)

    BANDS[first] * 10 + BANDS[second]
  end
end
