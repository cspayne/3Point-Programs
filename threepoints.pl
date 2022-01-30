/*objects*/

point2d(_,_).

/*line queries*/

vertical(point2d(X,_), point2d(X,_)).
horizontal(point2d(_,Y), point2d(_,Y)).

line(point2d(X,Y), point2d(A,B), point2d(C,D)):- ((B-Y)/(A-X)) =:= ((D-B)/(C-A)), ((D-B)/(C-A)) =:= ((Y-D)/(X-C)).

/*triange queries*/

triangle(point2d(X,Y), point2d(A,B), point2d(C,D)):- abs((X*(B-D)+A*(D-Y)+C*(Y-B))/2) > 0.

isosceles(point2d(X,Y), point2d(A,B), point2d(C,D)):- sqrt(((X-A)^2)+((Y-B)^2)) =:= sqrt(((A-C)^2)+((B-D)^2));
                                                      sqrt(((X-A)^2)+((Y-B)^2)) =:= sqrt(((C-X)^2)+((D-Y)^2));
                                                      sqrt(((A-C)^2)+((B-D)^2)) =:= sqrt(((C-X)^2)+((D-Y)^2)).

equilateral(point2d(X,Y), point2d(A,B), point2d(C,D)):- abs((sqrt(((X-A)^2)+((Y-B)^2)) - sqrt(((A-C)^2)+((B-D)^2)))) < 0.00001,
                                                        abs((sqrt(((X-A)^2)+((Y-B)^2)) - sqrt(((C-X)^2)+((D-Y)^2)))) < 0.00001,
                                                        abs((sqrt(((A-C)^2)+((B-D)^2)) - sqrt(((C-X)^2)+((D-Y)^2)))) < 0.00001.

right(point2d(X,Y), point2d(A,B), point2d(C,D)):- sqrt(((X-A)^2)+((Y-B)^2) + ((A-C)^2)+((B-D)^2)) =:= sqrt(((C-X)^2)+((D-Y)^2));
                                                  sqrt(((X-A)^2)+((Y-B)^2) + ((C-X)^2)+((D-Y)^2)) =:= sqrt(((A-C)^2)+((B-D)^2));
                                                  sqrt(((A-C)^2)+((B-D)^2) + ((C-X)^2)+((D-Y)^2)) =:= sqrt(((X-A)^2)+((Y-B)^2)).

scalene(point2d(X,Y), point2d(A,B), point2d(C,D)):- sqrt(((X-A)^2)+((Y-B)^2)) =\= sqrt(((A-C)^2)+((B-D)^2)),
                                                    sqrt(((X-A)^2)+((Y-B)^2)) =\= sqrt(((C-X)^2)+((D-Y)^2)),
                                                    sqrt(((A-C)^2)+((B-D)^2)) =\= sqrt(((C-X)^2)+((D-Y)^2)).

acute(point2d(X,Y), point2d(A,B), point2d(C,D)):- (((A-C)^2)+((B-D)^2) + ((C-X)^2)+((D-Y)^2)) > ((X-A)^2)+((Y-B)^2),
                                                  (((A-C)^2)+((B-D)^2) + ((X-A)^2)+((Y-B)^2)) > ((C-X)^2)+((D-Y)^2),
                                                  (((X-A)^2)+((Y-B)^2) + ((C-X)^2)+((D-Y)^2)) > ((A-C)^2)+((B-D)^2).

obtuse(point2d(X,Y), point2d(A,B), point2d(C,D)):- not(right(point2d(X,Y), point2d(A,B), point2d(C,D))),
                                                   not(acute(point2d(X,Y), point2d(A,B), point2d(C,D))).
