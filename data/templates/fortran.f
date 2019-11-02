#!/usr/bin/env scriptisto

! scriptisto-begin
! script_src: script.f
! build_cmd: gfortran -ffree-form -O2 script.f -o ./script
! scriptisto-end

program script
   print '("Hello, Fortran!", I0)'
end program script
