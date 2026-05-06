# frozen_string_literal: true

module Blackjack
  CARD_VALUES = {
    'ace' => 11,
    'two' => 2,  'three' => 3,  'four'  => 4, 'five' => 5,
    'six' => 6,  'seven' => 7,  'eight' => 8, 'nine' => 9,
    'ten' => 10, 'jack'  => 10, 'queen' => 10, 'king' => 10
  }.freeze

  def self.parse_card(card)
    CARD_VALUES.fetch(card, 0)
  end

  def self.card_range(card1, card2)
    case parse_card(card1) + parse_card(card2)
    when 4..11  then 'low'
    when 12..16 then 'mid'
    when 17..20 then 'high'
    when 21     then 'blackjack'
    end
  end

  def self.first_turn(card1, card2, dealer_card)
    range  = card_range(card1, card2)
    dealer = parse_card(dealer_card)

    case
    when card1 == 'ace' && card2 == 'ace' then 'P'
    when range == 'blackjack'             then dealer_could_blackjack?(dealer_card) ? 'S' : 'W'
    when range == 'high'                  then 'S'
    when range == 'mid' && dealer < 7     then 'S'
    else                                       'H'
    end
  end

  def self.dealer_could_blackjack?(card)
    %w[ace ten jack queen king].include?(card)
  end
  private_class_method :dealer_could_blackjack?
end
