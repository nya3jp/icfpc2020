(use srfi-13)

(define (add) (lambda (x) (lambda (y) (delay (+ (force x) (force y))))))
(define (lt) (lambda (x) (lambda (y) (delay (if (< (force x) (force y)) (t) (f))))))
(define (mul) (lambda (x) (lambda (y) (delay (* (force x) (force y))))))
(define (neg) (lambda (x) (delay (- (force x)))))
(define (i) (lambda (x) x))
(define (div) (lambda (x) (lambda (y) (delay (quotient (force x) (force y))))))

(define (isnil) (lambda (x) ((force x) (lambda (x) (lambda (y) (f))))))
(define (nil) (lambda (x) (t)))

; (define (eq) (lambda (x) (lambda (y) (delay (= (force x) (force y))))))
; (define (mycar) (lambda (x) (delay (car (force x)))))
; (define (mycdr) (lambda (x) (delay (cdr (force x)))))
; (define (mycons) (lambda (x) (lambda (y) (delay (cons x y)))))
; (define (t) (delay 't))
; (define (nil) (delay 'nil))

(define (eq) (lambda (x) (lambda (y) (delay (if (= (force x) (force y)) (t) (f))))))
(define (mycar) (lambda (c) (delay (force ((force c) (t))))))
(define (mycdr) (lambda (c) (delay (force ((force c) (f))))))
(define (mycons) (lambda (x) (lambda (y) (lambda (z) (delay (force ((force ((force z) x)) y)))))))
(define (t) (lambda (x) (lambda (y) x)))
(define (f) (lambda (x) (lambda (y) y)))

(define (b) (lambda (x) (lambda (y) (lambda (z) (delay (force ((force x) ((force y) z))))))))
(define (c) (lambda (x) (lambda (y) (lambda (z) (delay (force ((force ((force x) z)) y)))))))
(define (s) (lambda (x) (lambda (y) (lambda (z) (delay (force ((force ((force x) z)) ((force y) z))))))))

(define (printout x) (print (serialize x)))
; (define (printout x) (printgalaxyresult x))

(define (printgalaxyresult x)
  (let ((result (serialize x)))
    (print (car result))
    (print (cadr result))
    (map printdrawcall (caddr result))))

(define (printdrawcall drawinst)
  (print
    (string-join (list "draw(["
    (string-join
      (map (lambda (point) (format "(~D, ~D)" (car point) (cdr point))) drawinst)
      ", ") "])"))))

(define (serialize x)
  (if (number? (force x)) (force x)

  (if (= 1 ((force ((force ((isnil) x)) 1)) 2)) '()

    (cons (serialize ((mycar) (force x)))
          (serialize ((mycdr) (force x)))))))
