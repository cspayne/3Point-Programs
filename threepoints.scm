;; Scheme Triangle Program
;; Written By Cypress Payne
;; For CSC 3310

;; creates point (x, y)

(define (make-point x-cor y-cor)
  (cons x-cor y-cor)

)

;; gets x

(define (get-x point)
  (car point)
)

;; gets y

(define (get-y point)
  (cdr point)
)

;; tests if three points create a line

(define (is-line point1 point2 point3)
  (= 0 (area point1 point2 point3))
)

;;finds distance between two points

(define (distance point1 point2)
  (define x1 (get-x point1))
  (define y1 (get-y point1))
  (define x2 (get-x point2))
  (define y2 (get-y point2))
  (sqrt (+ (expt (- x2 x1) 2) (expt (- y2 y1) 2)))
)

;; finds perimeter of a triangle

(define (perimeter point1 point2 point3)
  (define dist12 (distance point1 point2))
  (define dist23 (distance point2 point3))
  (define dist31 (distance point3 point1))
  (+ dist31 (+ dist23 dist12))
)

;; finds area of a triangle

(define (area point1 point2 point3)
  (define p (/ (perimeter point1 point2 point3) 2))
  (define dist12 (distance point1 point2))
  (define dist23 (distance point2 point3))
  (define dist31 (distance point3 point1))
  (define a (- p dist12))
  (define b (- p dist23))
  (define c (- p dist31))
  (abs (sqrt (* p (* a (* b c)))))
)

;; finds, in radians,  one inner angle of a triangle

(define (lawofcos point1 point2 point3)
  (define dist12 (distance point1 point2))
  (define dist23 (distance point2 point3))
  (define dist31 (distance point3 point1))
  (define cos1 (/ (- (+ (expt dist12 2) (expt dist23 2)) (expt dist31 2)) (* (* dist12 dist23) 2)))
  (acos cos1)
)

;; evaluates triangle given three points

(define (calculate-triangle point1 point2 point3)
 (display "Side 1 = ")
 (display (/ (round (* 1000 (distance point1 point2))) 1000))
 (newline)
 (display "Side 2 = ")
 (display (/ (round (* 1000 (distance point2 point3))) 1000))
 (newline)
 (display "Side 3 = ")
 (display (/ (round (* 1000 (distance point3 point1))) 1000))

 (newline)
 (display "Perimeter = ")
 (/ (round (* 1000 (perimeter point1 point2 point3))) 1000)
 (display(/ (round (* 1000 (perimeter point1 point2 point3))) 1000))

 (newline)
 (display "Area = ")
 (display(/ (round (* 1000 (area point1 point2 point3))) 1000))

 (newline)
 (display "Angle 1 = ")
 (display(/ (round (* 100000 (lawofcos point2 point3 point1))) 100000))
 (display "\t")
 (display (/ (round (* 100000 (* (/ 180 3.14159265359) (lawofcos point2 point3 point1)))) 100000))
 (newline)
 (display "Angle 2 = ")
 (display (/ (round (* 100000 (lawofcos point1 point2 point3))) 100000))
 (display "\t")
 (display (/ (round (* 100000 (* (/ 180 3.14159265359) (lawofcos point1 point2 point3)))) 100000))
 (newline)
 (display "Angle 3 = ")
 (display (/ (round (* 100000 (lawofcos point3 point1 point2))) 100000))
 (display "\t")
 (display (/ (round (* 100000 (* (/ 180 3.14159265359) (lawofcos point3 point1 point2)))) 100000))
 (newline)
)
