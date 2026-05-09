class Acronym
  def self.abbreviate(phrase)
    phrase.scan(/[a-z]+(?:'[a-z]+)?/i).map { |word| word[0] }.join.upcase
  end
end
