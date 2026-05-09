class Acronym
  def self.abbreviate(phrase)
    phrase.scan(/([a-z])[a-z']*/i).join.upcase
  end
end
