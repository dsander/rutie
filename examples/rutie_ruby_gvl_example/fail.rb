require 'rutie_ruby_gvl_example'
class Test
  def self.many_args(*args)
    args.each do |a|
      puts a
      # GC.start
    end
  end
end
ret = RutieExample.many_arguments
puts ret
