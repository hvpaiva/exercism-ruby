class HighScores
  attr_reader :scores

  def initialize(scores)
    @scores = scores
  end

  def latest = scores.last

  def personal_best = scores.max

  def personal_top_three = scores.max(3)

  def latest_is_personal_best? = latest == personal_best
end
