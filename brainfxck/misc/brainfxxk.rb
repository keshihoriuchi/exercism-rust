# coding: utf-8

def brainfxxk(src)
  buf = [0] * 30000
  ptr = 0
  bracket_stack = []
  len = src.length
  pc = 0
  while pc < len do
    c = src[pc]
    case c
    when '>'
      ptr += 1
    when '<'
      ptr -= 1
    when '+'
      buf[ptr] += 1
    when '-'
      buf[ptr] -= 1
    when '.'
      putc(buf[ptr])
    when ','
      buf[ptr] = getc
    when '['
      if buf[ptr] == 0
        while src[pc] != ']' do
          pc += 1
        end
      else
        bracket_stack.push(pc)
      end
    when ']'
      pc = bracket_stack.pop
      next
    else
      # ignore
    end
    pc += 1
  end
end

brainfxxk <<EOS
>+++++++++[<++++++++>-]<.>+++++++[<++++>-]<+.+++++++..+++.[-]>++++++++[<++
++>-]<.>+++++++++++[<+++++>-]<.>++++++++[<+++>-]<.+++.------.--------.[-]>
++++++++[<++++>-]<+.[-]++++++++++.
EOS
