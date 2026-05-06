module Chess
  RANKS = 1..8
  FILES = 'A'..'H'

  def self.valid_square?(rank, file)
    RANKS.cover?(rank) && FILES.to_a.include?(file)
  end

  def self.nickname(first_name, last_name)
    (first_name[..1] + last_name[-2..]).upcase
  end

  def self.move_message(first_name, last_name, square)
    nick = nickname(first_name, last_name)
    rank, file = square[1].to_i, square[0]

    if valid_square?(rank, file)
      "#{nick} moved to #{square}"
    else
      "#{nick} attempted to move to #{square}, but that is not a valid square"
    end
  end
end
