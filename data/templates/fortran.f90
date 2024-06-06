#!/usr/bin/env scriptisto

! scriptisto-begin
! script_src: script.f90
! build_cmd: gfortran script.f90 -o ./script
! scriptisto-end

program script
   implicit none
   print *, "Hello, Fortran!"
end program script
