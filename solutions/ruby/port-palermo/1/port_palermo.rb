module Port
  IDENTIFIER = :PALE

  def self.get_identifier(city)
    city.slice(0, 4).upcase.to_sym
  end

  def self.get_terminal(ship_identifier)
    ship_identifier.to_s.start_with?("OIL", "GAS") ? :A : :B
  end
end
