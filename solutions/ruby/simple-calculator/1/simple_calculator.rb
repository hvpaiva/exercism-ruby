class SimpleCalculator
  class UnsupportedOperation < StandardError; end

  ALLOWED_OPERATIONS = %w[+ / *].freeze

  def self.calculate(first_operand, second_operand, operation)
    unless first_operand.is_a?(Integer) && second_operand.is_a?(Integer)
      raise ArgumentError
    end

    unless ALLOWED_OPERATIONS.include?(operation)
      raise UnsupportedOperation
    end

    return "Division by zero is not allowed." if operation == "/" && second_operand.zero?

    result = first_operand.public_send(operation, second_operand)

    "#{first_operand} #{operation} #{second_operand} = #{result}"
  end
end
