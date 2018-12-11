import sys
import matplotlib.pyplot as plt
from matplotlib import axes

if __name__ == '__main__':
    fig = plt.figure()
    ax = fig.add_subplot(1, 1, 1)

    # Move left y-axis and bottim x-axis to centre, passing through (0,0)
    ax.spines['left'].set_position('center')
    ax.spines['bottom'].set_position('center')

    # Eliminate upper and right axes
    ax.spines['right'].set_color('none')
    ax.spines['top'].set_color('none')

    # Show ticks in the left and lower axes only
    ax.xaxis.set_ticks_position('bottom')
    ax.yaxis.set_ticks_position('left')

    plt.grid(True)

    if len(sys.argv) >= 3:
        def str_to_flt_array(string):
            arr = []
            for nstr in string.split(','):
                arr.append(float(nstr))
            return arr
        x = str_to_flt_array(sys.argv[1])
        y = str_to_flt_array(sys.argv[2])

        plt.plot(x, y)
    else:
        plt.plot([-10,-9,-7,-6,-5,-4,-3,-2,-1,0,1,2,3,4,5,6,7,8,9,10], [-10,-9,-7,-6,-5,-4,-3,-2,-1,0,1,2,3,4,5,6,7,8,9,10])
        #plt.axis([-10, 10, -10, 10])
    plt.show()
